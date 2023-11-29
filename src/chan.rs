use crate::AsyncQueue;
use futures_lite::prelude::*;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll};

#[inline]
pub fn chan<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    assert!(capacity > 0, "capacity must not be 0");

    let inner = AsyncQueue::new(capacity);

    let rx_inner = Rc::new(RefCell::new(inner));
    let tx_inner = Rc::downgrade(&rx_inner);
    let tx = Sender(tx_inner);
    let rx = Receiver(rx_inner);

    (tx, rx)
}

pub struct Sender<T>(Weak<RefCell<AsyncQueue<T>>>);
pub struct Receiver<T>(Rc<RefCell<AsyncQueue<T>>>);

impl<T> Clone for Sender<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Drop for Sender<T> {
    #[inline]
    fn drop(&mut self) {
        match self.0.upgrade() {
            None => {}
            Some(inner) => {
                inner.borrow_mut().wake();
            }
        }
    }
}

impl<T> Sender<T> {
    #[inline]
    pub fn closed(&self) -> bool {
        self.0.upgrade().is_none()
    }

    #[inline]
    pub fn send(&self, value: T) -> Result<(), Error> {
        match self.0.upgrade() {
            None => Err(Error::Closed),
            Some(q) => q.borrow_mut().enqueue(value).map_err(|_| Error::Full),
        }
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(item) = self.0.borrow_mut().poll_next(cx) {
            debug_assert!(item.is_some());
            Poll::Ready(item)
        } else if Rc::weak_count(&self.0) == 0 {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Closed,
    Full,
}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Closed => write!(f, "channel closed"),
            Error::Full => write!(f, "no available capacity"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn i32() {
        let (tx, mut rx) = chan::<i32>(usize::MAX);

        assert!(tx.send(0).is_ok());
        assert_eq!(rx.next().await, Some(0));

        assert!(tx.send(1).is_ok());
        assert!(tx.send(-1).is_ok());
        assert_eq!(rx.next().await, Some(1));
        assert_eq!(rx.next().await, Some(-1));
    }

    #[wasm_bindgen_test]
    async fn drop_tx() {
        let (tx, mut rx) = chan::<i32>(usize::MAX);
        assert!(tx.send(1).is_ok());
        drop(tx);
        assert_eq!(rx.next().await, Some(1));
        assert_eq!(rx.next().await, None);
    }

    #[wasm_bindgen_test]
    async fn send_to_closed_tx() {
        let (tx, rx) = chan::<i32>(usize::MAX);
        drop(rx);
        assert!(tx.send(1).is_err());
    }

    #[wasm_bindgen_test]
    async fn capacity() {
        let (tx, mut rx) = chan::<i32>(3);
        assert!(tx.send(1).is_ok());
        assert!(tx.send(2).is_ok());
        assert!(tx.send(3).is_ok());
        assert!(tx.send(4).is_err());
        assert_eq!(rx.next().await, Some(1));
        assert!(tx.send(5).is_ok());
        assert!(tx.send(6).is_err());
    }
}
