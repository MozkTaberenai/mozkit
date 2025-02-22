use crate::chan;
use futures_lite::prelude::*;
use slab::Slab;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll};

pub struct Emitter<T> {
    tx_slab: Rc<RefCell<Slab<chan::Sender<T>>>>,
    capacity: usize,
}

impl<T> Default for Emitter<T> {
    #[inline]
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T> Emitter<T> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            tx_slab: Default::default(),
            capacity,
        }
    }
}

impl<T: Clone> Emitter<T> {
    #[inline]
    pub fn emit(&self, value: T) {
        for (_, tx) in self.tx_slab.borrow().iter() {
            if tx.send(value.clone()).is_err() {
                log::error!("fail to send");
            }
        }
    }

    #[inline]
    pub fn receive(&self) -> impl Stream<Item = T> + use<T> {
        let (tx, rx) = chan(self.capacity);
        let slab_key = self.tx_slab.borrow_mut().insert(tx);
        let tx_slab = Rc::downgrade(&self.tx_slab);
        Receiver {
            rx,
            tx_slab,
            slab_key,
        }
    }
}

struct Receiver<T> {
    rx: chan::Receiver<T>,
    tx_slab: Weak<RefCell<Slab<chan::Sender<T>>>>,
    slab_key: usize,
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next(cx)
    }
}

impl<T> Drop for Receiver<T> {
    #[inline]
    fn drop(&mut self) {
        if let Some(tx_slab) = self.tx_slab.upgrade() {
            tx_slab.borrow_mut().remove(self.slab_key);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    async fn test() {
        let emitter = Emitter::new(usize::MAX);
        let mut receiver = emitter.receive();

        emitter.emit(1);
        assert_eq!(receiver.next().await, Some(1));
        drop(emitter);
        assert_eq!(receiver.next().await, None);
    }
}
