extern crate alloc;

use core::fmt;
use core::fmt::Write as _;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe extern "C" {
        #[link_name = "$panic"]
        fn ff() -> !;
    }
    if option_env!("DEBUG").is_some() {
        let mut buf = WriteBuf::new();

        write!(buf, "{info}").ok();

        web::error!(&js::String::from(buf.as_str()));
    }
    unsafe { ff() }
}

struct WriteBuf {
    buf: [u8; 1024],
    cursor: usize,
}

impl WriteBuf {
    fn new() -> Self {
        Self {
            buf: [0; 1024],
            cursor: 0,
        }
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.cursor]) }
    }

    fn capacity(&self) -> usize {
        self.buf.len()
    }

    fn remaining(&self) -> usize {
        self.capacity() - self.cursor
    }
}

impl fmt::Write for WriteBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len().min(self.remaining());
        self.buf[self.cursor..][..len].copy_from_slice(&bytes[..len]);
        self.cursor += len;

        Ok(())
    }
}
