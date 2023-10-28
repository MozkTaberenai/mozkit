use futures_lite::Stream;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll, Waker};

pub struct Channel<T> {
    pub tx: ChannelTx<T>,
    pub rx: ChannelRx<T>,
}

pub struct ChannelTx<T>(Weak<RefCell<Inner<T>>>);
pub struct ChannelRx<T>(Rc<RefCell<Inner<T>>>);

struct Inner<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
    capacity: usize,
}

#[derive(Debug)]
pub enum ChannelSendError<T> {
    Full(T),
    Closed(T),
}

impl<T> std::fmt::Display for ChannelSendError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelSendError::Full(..) => write!(f, "no available capacity"),
            ChannelSendError::Closed(..) => write!(f, "channel closed"),
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for ChannelSendError<T> {}

pub fn mpsc<T>(capacity: usize) -> (ChannelTx<T>, ChannelRx<T>) {
    Channel::new(capacity).split_into()
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must not be 0");

        let inner = Inner {
            queue: VecDeque::with_capacity(1),
            waker: None,
            capacity,
        };

        let rx = Rc::new(RefCell::new(inner));
        let tx = Rc::downgrade(&rx);
        let tx = ChannelTx(tx);
        let rx = ChannelRx(rx);

        Self { tx, rx }
    }

    pub fn split_into(self) -> (ChannelTx<T>, ChannelRx<T>) {
        (self.tx, self.rx)
    }
}

impl<T> Clone for ChannelTx<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Drop for ChannelTx<T> {
    fn drop(&mut self) {
        match self.0.upgrade() {
            None => {}
            Some(inner) => {
                let Inner { ref mut waker, .. } = *inner.borrow_mut();
                if let Some(waker) = waker.take() {
                    waker.wake();
                }
            }
        }
    }
}

impl<T> ChannelTx<T> {
    pub fn closed(&self) -> bool {
        self.0.upgrade().is_none()
    }

    pub fn send(&self, value: T) -> Result<(), ChannelSendError<T>> {
        match self.0.upgrade() {
            None => Err(ChannelSendError::Closed(value)),
            Some(inner) => {
                let Inner {
                    ref mut queue,
                    ref mut waker,
                    capacity,
                } = *inner.borrow_mut();

                if queue.len() < capacity {
                    queue.push_back(value);

                    if let Some(waker) = waker.take() {
                        waker.wake();
                    }

                    Ok(())
                } else {
                    Err(ChannelSendError::Full(value))
                }
            }
        }
    }
}

impl<T> Stream for ChannelRx<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Inner {
            ref mut queue,
            ref mut waker,
            ..
        } = *self.0.borrow_mut();
        match queue.pop_front() {
            Some(item) => Poll::Ready(Some(item)),
            None => {
                match Rc::weak_count(&self.0) {
                    0 => {
                        // all Tx droped
                        Poll::Ready(None)
                    }
                    _ => {
                        waker.replace(cx.waker().clone());
                        Poll::Pending
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    use crate::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn i32() {
        let mut chan = Channel::<i32>::new(usize::MAX);

        assert!(chan.tx.send(0).is_ok());
        assert_eq!(chan.rx.next().await, Some(0));

        assert!(chan.tx.send(1).is_ok());
        assert!(chan.tx.send(-1).is_ok());
        assert_eq!(chan.rx.next().await, Some(1));
        assert_eq!(chan.rx.next().await, Some(-1));
    }

    #[wasm_bindgen_test]
    async fn drop_tx() {
        let mut chan = Channel::<i32>::new(usize::MAX);
        assert!(chan.tx.send(1).is_ok());
        drop(chan.tx);
        assert_eq!(chan.rx.next().await, Some(1));
        assert_eq!(chan.rx.next().await, None);
    }

    #[wasm_bindgen_test]
    async fn send_to_closed_tx() {
        let chan = Channel::<i32>::new(usize::MAX);
        drop(chan.rx);
        assert!(matches!(chan.tx.send(1), Err(ChannelSendError::Closed(1))));
    }

    #[wasm_bindgen_test]
    async fn capacity() {
        let mut chan = Channel::<i32>::new(3);
        assert!(chan.tx.send(1).is_ok());
        assert!(chan.tx.send(2).is_ok());
        assert!(chan.tx.send(3).is_ok());
        assert!(matches!(chan.tx.send(4), Err(ChannelSendError::Full(4))));
        assert_eq!(chan.rx.next().await, Some(1));
        assert!(chan.tx.send(5).is_ok());
        assert!(matches!(chan.tx.send(6), Err(ChannelSendError::Full(6))));
    }
}
