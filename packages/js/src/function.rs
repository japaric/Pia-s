use crate::{IsFunction, IsValue, Value};

inheritance!(Function: Value);

impl<T> From<T> for Function
where
    T: IsFunction,
{
    fn from(value: T) -> Self {
        unsafe extern "C" {
            #[link_name = "$Function$from"]
            fn ff(table_index: u32) -> Function;
        }

        unsafe { ff(value.table_index()) }
    }
}

impl IsFunction for fn() {
    fn table_index(&self) -> u32 {
        *self as usize as u32
    }
}

impl<A> IsFunction for fn(A)
where
    A: IsValue,
{
    fn table_index(&self) -> u32 {
        *self as usize as u32
    }
}
