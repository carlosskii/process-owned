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
    ops::Deref
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
    value: Rc<RefCell<T>>
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
            value: Rc::new(RefCell::new(value))
        }
    }
}

impl<T> Deref for ProcessOwned<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}