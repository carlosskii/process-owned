#![warn(missing_docs)]

//! This crate provides the `ProcessOwned` type, a value
//! that shares its lifetime with the process *unless* it
//! can be optimally freed earlier than that.
//! 
//! Internally, `ProcessOwned` uses the `Rc` type to
//! ensure that the value is only dropped when the last
//! owner is dropped. The specific implementation is
//! subject to change for performance reasons.

use std::{
    rc::Rc,
    cell::RefCell,
    ops::Deref,
    alloc::{alloc, Layout}
};

/// A value that is owned by the process itself.
/// 
/// The lifetime of a value is tied to its owner. When the
/// owner is dropped, the value is dropped as well. This
/// object uses the `Rc` type internally to ensure that
/// the value is only dropped when the last owner is dropped.
/// The specific implementation is subject to change for
/// performance reasons.
/// 
/// When a `ProcessOwned` is initialized as a global static
/// variable with the `lazy_static` crate, it will never be
/// dropped. This allows values to easily and clearly share
/// the scope of the entire process.
#[derive(Clone, Debug)]
pub struct ProcessOwned<T> {
    value: Rc<T>
}

impl<T> ProcessOwned<T> {
    /// Create a new `ProcessOwned` value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use process_owned::ProcessOwned;
    /// 
    /// let value = ProcessOwned::new(5);
    /// assert_eq!(*value, 5);
    /// ```
    pub fn new(value: T) -> Self {
        ProcessOwned {
            value: Rc::new(value)
        }
    }
}

impl<T> Deref for ProcessOwned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// A mutable version of `ProcessOwned`.
/// 
/// This type is identical to `ProcessOwned` except that
/// it provides mutability. This is done by using a
/// `RefCell` internally.
#[derive(Clone, Debug)]
pub struct ProcessOwnedMut<T> {
    value: Rc<RefCell<T>>
}

impl<T> ProcessOwnedMut<T> {
    /// Create a new `ProcessOwnedMut` value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use process_owned::ProcessOwnedMut;
    /// 
    /// let mut value = ProcessOwnedMut::new(5);
    /// *value.borrow_mut() = 10;
    /// assert_eq!(*value.borrow(), 10);
    /// ```
    pub fn new(value: T) -> Self {
        ProcessOwnedMut {
            value: Rc::new(RefCell::new(value))
        }
    }
}

impl<T> Deref for ProcessOwnedMut<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// A value that is truly owned by the process itself.
/// 
/// Using this type is similar to using a `ProcessOwned`
/// with `lazy_static`, except the value can only be dropped
/// when the process terminates. This can lead to memory
/// leaks, but it can also be useful for values that are
/// used throughout the entire process.
#[derive(Clone, Debug)]
pub struct Immortal<T> {
    value: *const T
}

impl<T> Immortal<T> {
    /// Creates a new `Immortal` value.
    pub unsafe fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = alloc(layout) as *mut T;
        *ptr = value;

        Immortal {
            value: ptr
        }
    }
}

impl<T> From<Immortal<T>> for *const T {
    fn from(immortal: Immortal<T>) -> Self {
        immortal.value
    }
}

impl<T> Deref for Immortal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}