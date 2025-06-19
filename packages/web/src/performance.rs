pub struct Performance;

impl Performance {
    pub fn now(&self) -> f64 {
        unsafe extern "C" {
            #[link_name = "$Performance$now"]
            fn ff() -> f64;
        }

        unsafe { ff() }
    }
}
