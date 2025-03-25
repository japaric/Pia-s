use crate::{Downcast, Object, Value};

inheritance!(Map: Object);

impl Map {
    pub fn get(&self, key: &Value) -> Option<Value> {
        call!(self, get, key)
    }

    pub fn entries(&self) -> crate::Iterator {
        unsafe { call!(self, entries).unwrap_unchecked().downcast() }
    }
}
