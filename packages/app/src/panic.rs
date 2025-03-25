extern crate alloc;

use alloc::string::ToString;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe extern "C" {
        #[link_name = "$panic"]
        fn ff() -> !;
    }
    if option_env!("DEBUG").is_some() {
        web::error!(&js::String::from(info.to_string().as_str()));
    }
    unsafe { ff() }
}
