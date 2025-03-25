use super::Object;

inheritance!(String: Object);

// TODO compile-time intern `'static` strings on the JS side to avoid conversions at runtime
impl From<&'_ str> for String {
    fn from(value: &'_ str) -> Self {
        unsafe extern "C" {
            #[link_name = "$String$from_str"]
            fn ff(str_ptr: u32, str_len: u32) -> String;
        }

        unsafe { ff(value.as_ptr() as u32, value.len() as u32) }
    }
}
