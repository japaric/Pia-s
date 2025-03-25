use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::{mem, ptr};

use tlsf::Tlsf;

use nosync::{Owned, Shared};

const SIZE: usize = 8 * 1024 / mem::size_of::<u32>();

pub fn initialize() {
    static MEMORY: Owned<[MaybeUninit<u32>; SIZE]> = Owned::new([MaybeUninit::uninit(); SIZE]);

    if let Some(memory) = MEMORY.take() {
        HEAP.initialize(memory)
    }
}

#[global_allocator]
static HEAP: Heap = Heap::new();

struct Heap {
    // FL=6 => MAX_ALLOC_SIZE = 1,984B
    inner: Shared<Tlsf<'static, 6>>,
}

impl Heap {
    const fn new() -> Self {
        Self {
            inner: Shared::new(Tlsf::empty()),
        }
    }

    fn initialize(&self, memory: &'static mut [MaybeUninit<u32>]) {
        self.inner.borrow_mut().initialize(memory)
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner
            .borrow_mut()
            .memalign(layout)
            .map(|slice| slice.as_mut_ptr().cast())
            .unwrap_or(ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if let Some(nn) = NonNull::new(ptr) {
            unsafe {
                self.inner.borrow_mut().free(nn.cast());
            }
        }
    }
}
