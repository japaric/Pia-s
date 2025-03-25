use core::num::NonZeroU32;

use crate::Object;

inheritance!(Uint8Array: Object);

impl Uint8Array {
    pub fn length(&self) -> u32 {
        unsafe { self.get(&"length".into()).unwrap_unchecked().to_u32() }
    }

    pub fn copy_to_slice(&self, dst: &mut [u8]) {
        unsafe extern "C" {
            #[link_name = "$Uint8Array$copy_to_slice"]
            fn ff(array: NonZeroU32, dst_ptr: u32, dst_len: u32);
        }

        unsafe { ff(self.index(), dst.as_mut_ptr() as u32, dst.len() as u32) }
    }
}
