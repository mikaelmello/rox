use std::{any::Any, mem};

use crate::heap::Object;

impl Object for String {
    fn size(&self) -> usize {
        mem::size_of::<String>() + self.capacity()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
