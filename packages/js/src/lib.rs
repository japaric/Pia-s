#![no_std]

#[macro_use]
mod macros;
mod array;
mod boolean;
mod function;
mod integer;
mod iterator;
mod map;
mod null;
mod object;
mod promise;
mod string;
mod typed_array;
mod value;

pub use array::Array;
pub use boolean::*;
pub use function::Function;
pub use integer::Integer;
pub use iterator::Iterator;
pub use map::Map;
pub use null::Null;
pub use object::Object;
pub use promise::Promise;
pub use string::String;
pub use typed_array::Uint8Array;
pub use value::Value;

pub trait Downcast<Subtype: IsValue> {
    fn downcast(self) -> Subtype;
}

pub trait Upcast: IsValue {
    type Supertype;

    fn upcast(self) -> Self::Supertype;
}

/// # Safety
/// Implementer must be a JS value
pub unsafe trait IsValue {}

pub trait IsFunction {
    fn table_index(&self) -> u32;
}
