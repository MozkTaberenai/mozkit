use crate::*;
use slab::Slab;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll, Waker};

#[derive(Clone)]
pub struct Event<T: Clone> {
    inner: Rc<RefCell<EventInner<T>>>,
}

struct EventInner<T> {
    listeners: Slab<Weak<RefCell<EventStreamInner<T>>>>,
    capacity: usize,
}

pub struct EventStream<T> {
    inner: Rc<RefCell<EventStreamInner<T>>>,
    event_inner: Weak<RefCell<EventInner<T>>>,
    index: usize,
}

struct EventStreamInner<T> {
    queue: VecDeque<T>,
    waker: Option<Waker>,
}

impl<T: Clone> Event<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must not be 0");

        let inner = EventInner {
            listeners: Slab::new(),
            capacity,
        };
        let inner = Rc::new(RefCell::new(inner));

        Self { inner }
    }

    #[track_caller]
    pub fn emit(&self, value: T) {
        if let Err((_ok, err)) = self.try_emit(value) {
            panic!("event emit failed on {err} listeners");
        }
    }

    pub fn try_emit(&self, value: T) -> Result<usize, (usize, usize)> {
        let mut ok = 0;
        let mut err = 0;

        let EventInner {
            ref listeners,
            capacity,
        } = *self.inner.borrow();

        for (_, listener) in listeners {
            let inner = listener
                .upgrade()
                .expect_throw("Should successfuly upgraded");

            let EventStreamInner {
                ref mut queue,
                ref mut waker,
            } = *inner.borrow_mut();

            if queue.len() < capacity {
                queue.push_back(value.clone());

                if let Some(waker) = waker.take() {
                    waker.wake();
                }

                ok += 1;
            } else {
                err += 1;
            }
        }

        match err {
            0 => Ok(ok),
            _ => Err((ok, err)),
        }
    }

    pub fn listen(&self) -> EventStream<T> {
        let inner = EventStreamInner {
            queue: VecDeque::with_capacity(1),
            waker: None,
        };
        let inner = Rc::new(RefCell::new(inner));

        let event_inner = Rc::downgrade(&self.inner);
        let index = self
            .inner
            .borrow_mut()
            .listeners
            .insert(Rc::downgrade(&inner));

        EventStream {
            inner,
            event_inner,
            index,
        }
    }

    pub fn listener_count(&self) -> usize {
        self.inner.borrow().listeners.len()
    }
}

impl<T: Clone> Default for Event<T> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T> Stream for EventStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut inner_mut = self.inner.borrow_mut();
        match inner_mut.queue.pop_front() {
            Some(item) => Poll::Ready(Some(item)),
            None => {
                if self.event_inner.upgrade().is_some() {
                    inner_mut.waker.replace(cx.waker().clone());
                    Poll::Pending
                } else {
                    // Event dropped
                    Poll::Ready(None)
                }
            }
        }
    }
}

impl<T: Clone> Drop for Event<T> {
    fn drop(&mut self) {
        for (_, listener) in &self.inner.borrow().listeners {
            let inner = listener
                .upgrade()
                .expect_throw("Should successfuly upgraded");

            let EventStreamInner { ref mut waker, .. } = *inner.borrow_mut();

            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }
}

impl<T> Drop for EventStream<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.event_inner.upgrade() {
            inner.borrow_mut().listeners.remove(self.index);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn i32() {
        let event = Event::<i32>::new(usize::MAX);
        assert_eq!(event.listener_count(), 0);
        let mut l1 = event.listen();
        assert_eq!(event.listener_count(), 1);
        assert_eq!(event.try_emit(1), Ok(1));
        assert_eq!(l1.next().await, Some(1));
        let mut l2 = event.listen();
        assert_eq!(event.listener_count(), 2);
        assert_eq!(event.try_emit(2), Ok(2));
        let mut l3 = event.listen();
        assert_eq!(event.listener_count(), 3);
        assert_eq!(event.try_emit(3), Ok(3));
        assert_eq!(l2.next().await, Some(2));
        assert_eq!(l1.next().await, Some(2));
        assert_eq!(l1.next().await, Some(3));
        assert_eq!(l2.next().await, Some(3));
        assert_eq!(l3.next().await, Some(3));
    }

    #[wasm_bindgen_test]
    async fn drop_event() {
        let event = Event::<i32>::new(usize::MAX);
        assert_eq!(event.listener_count(), 0);
        let mut listener = event.listen();
        assert_eq!(event.listener_count(), 1);
        assert_eq!(event.try_emit(1), Ok(1));
        drop(event);
        assert_eq!(listener.next().await, Some(1));
        assert_eq!(listener.next().await, None);
    }

    #[wasm_bindgen_test]
    async fn drop_listener() {
        let event = Event::<i32>::new(usize::MAX);
        assert_eq!(event.listener_count(), 0);
        let mut l1 = event.listen();
        assert_eq!(event.listener_count(), 1);
        let l2 = event.listen();
        assert_eq!(event.listener_count(), 2);
        assert_eq!(event.try_emit(1), Ok(2));
        assert_eq!(l1.next().await, Some(1));
        drop(l2);
        assert_eq!(event.listener_count(), 1);
        assert_eq!(event.try_emit(2), Ok(1));
        let mut l3 = event.listen();
        assert_eq!(event.listener_count(), 2);
        assert_eq!(event.try_emit(3), Ok(2));
        assert_eq!(l1.next().await, Some(2));
        assert_eq!(l3.next().await, Some(3));
        assert_eq!(l1.next().await, Some(3));
    }

    #[wasm_bindgen_test]
    async fn capacity() {
        let event = Event::<i32>::new(3);
        let mut l1 = event.listen(); // l1:0/3
        assert_eq!(event.try_emit(1), Ok(1)); // l1:1/3
        assert_eq!(event.try_emit(2), Ok(1)); // l1:2/3
        let mut l2 = event.listen(); // l2:0/3
        assert_eq!(event.try_emit(3), Ok(2)); // l1: 3/3, l2: 1/3
        assert_eq!(event.try_emit(4), Err((1, 1))); // l1 3!/3, l2: 2/3
        assert_eq!(l1.next().await, Some(1)); // l1: 2/3, l2: 2/3
        assert_eq!(l2.next().await, Some(3)); // l1: 2/3, l2: 1/3
        assert_eq!(event.try_emit(5), Ok(2)); // l1: 3/3, l2: 2/3
        assert_eq!(event.try_emit(6), Err((1, 1))); // l1: 3!/3, l2: 3/3
        assert_eq!(event.try_emit(7), Err((0, 2))); // l1: 3!/3, l2: 3!/3
    }
}
