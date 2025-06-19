use core::num::NonZero;

use crate::{Downcast as _, Value};

inheritance!(Float: Value);

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        unsafe extern "C" {
            #[link_name = "$Float$from_f64"]
            fn ff(float: f64) -> Value;
        }

        unsafe { ff(value).downcast() }
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> f64 {
        unsafe extern "C" {
            #[link_name = "$Float$to_f64"]
            fn ff(float: NonZero<u32>) -> f64;
        }

        unsafe { ff(value.index()) }
    }
}
