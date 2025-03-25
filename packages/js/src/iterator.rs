use crate::{Downcast as _, Object, Value};

inheritance!(Iterator: Object);

impl Iterator {
    fn js_next(&self) -> IteratorNext {
        unsafe { call!(self, next).unwrap_unchecked().downcast() }
    }
}

impl core::iter::Iterator for Iterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.js_next().value()
    }
}

inheritance!(IteratorNext: Object);

impl IteratorNext {
    fn value(&self) -> Option<Value> {
        self.get(&"value".into())
    }
}
