//! SPuR: Subscribe Publish React

#![no_std]

extern crate alloc;

pub use spur_macros::{Message, subscriptions};

#[doc(hidden)]
pub mod internal;

pub trait Message {}

pub trait Publish<M: Message> {
    fn publish(message: M);
}

pub trait React<M: Message> {
    fn react(&mut self, message: M);
}
