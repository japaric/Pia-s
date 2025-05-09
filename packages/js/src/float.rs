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
