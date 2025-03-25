use core::marker::PhantomData;

use crate::{Function, IsValue, Object, Value};

// changes applied to expansion of
// inheritance!(Promise; Object, Value);

// #[derive(Clone)]
#[repr(transparent)]
pub struct Promise<T>
where
    T: IsValue, // added
{
    inner: Object,
    _result: PhantomData<T>, // added
}

impl<T> Promise<T>
where
    T: IsValue,
{
    pub fn then2(&self, fulfilled: fn(T), rejected: fn()) {
        call!(
            self,
            then,
            &Function::from(fulfilled),
            &Function::from(rejected)
        );
    }
}

impl<T> core::ops::Deref for Promise<T>
where
    T: IsValue,
{
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl<T> IsValue for Promise<T> where T: IsValue {}

impl<T> crate::Downcast<Promise<T>> for Object
where
    T: IsValue,
{
    fn downcast(self) -> Promise<T> {
        unsafe { core::mem::transmute(self) }
    }
}

impl<T> crate::Downcast<Promise<T>> for Value
where
    T: IsValue,
{
    fn downcast(self) -> Promise<T> {
        unsafe { core::mem::transmute(self) }
    }
}
