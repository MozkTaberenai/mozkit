use futures_lite::prelude::*;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pin_project_lite::pin_project! {
    #[derive(Debug)]
    pub struct AsyncQueue<T> {
        queue: VecDeque<T>,
        waker: Option<Waker>,
        capacity: usize,
    }
}

impl<T> Default for AsyncQueue<T> {
    #[inline]
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T> AsyncQueue<T> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(1),
            waker: None,
            capacity,
        }
    }

    #[inline]
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.queue.len() < self.capacity {
            self.queue.push_back(value);
            self.wake();
            Ok(())
        } else {
            Err(value)
        }
    }

    #[inline]
    pub fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Stream for AsyncQueue<T> {
    type Item = T;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let proj = self.project();
        match proj.queue.pop_front() {
            Some(item) => Poll::Ready(Some(item)),
            None => {
                proj.waker.replace(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    async fn i32() {
        let mut q = AsyncQueue::new(usize::MAX);

        assert!(q.enqueue(0).is_ok());

        assert_eq!(q.next().await, Some(0));

        assert!(q.enqueue(1).is_ok());
        assert!(q.enqueue(-1).is_ok());

        assert_eq!(q.next().await, Some(1));
        assert_eq!(q.next().await, Some(-1));
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    async fn capacity() {
        let mut q = AsyncQueue::new(3);
        assert!(q.enqueue(1).is_ok());
        assert!(q.enqueue(2).is_ok());
        assert!(q.enqueue(3).is_ok());
        assert!(q.enqueue(4).is_err());
        assert_eq!(q.next().await, Some(1));
        assert!(q.enqueue(5).is_ok());
        assert!(q.enqueue(6).is_err());
    }
}
