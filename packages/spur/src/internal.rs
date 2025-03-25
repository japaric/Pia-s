use alloc::collections::vec_deque::VecDeque;

pub use nosync::Shared;
pub use web::queue_microtask;

use crate::Message;

pub struct Queue<T>
where
    T: Message,
{
    inner: Shared<VecDeque<T>>,
}

impl<T> Queue<T>
where
    T: Message,
{
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            inner: Shared::new(VecDeque::new()),
        }
    }

    pub fn enqueue(&self, message: T) {
        self.inner.borrow_mut().push_back(message);
    }

    pub fn dequeue(&self) -> Option<T> {
        self.inner.borrow_mut().pop_front()
    }
}
