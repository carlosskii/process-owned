use std::{rc::Rc, ops::Deref};


pub struct ProcessOwned<T> {
    pub value: Rc<T>
}

impl<T> ProcessOwned<T> {
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