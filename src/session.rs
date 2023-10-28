use crate::*;
use slab::Slab;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll, Waker};

type Watchers = Slab<Weak<RefCell<SessionWatcherInner>>>;

#[derive(Clone, Default)]
pub struct Session {
    inner: Rc<RefCell<SessionInner>>,
}

#[derive(Default)]
struct SessionInner {
    watchers: Watchers,
}

struct SessionWatcher {
    inner: Rc<RefCell<SessionWatcherInner>>,
    session_inner: Weak<RefCell<SessionInner>>,
    index: usize,
}

impl Future for SessionWatcher {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.session_inner.upgrade().is_some() {
            true => {
                self.inner.borrow_mut().waker.replace(cx.waker().clone());
                Poll::Pending
            }
            false => Poll::Ready(()),
        }
    }
}

struct SessionWatcherInner {
    waker: Option<Waker>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    fn watch(&self) -> SessionWatcher {
        let inner = SessionWatcherInner { waker: None };
        let inner = Rc::new(RefCell::new(inner));

        let session_inner = Rc::downgrade(&self.inner);
        let index = self
            .inner
            .borrow_mut()
            .watchers
            .insert(Rc::downgrade(&inner));

        SessionWatcher {
            inner,
            session_inner,
            index,
        }
    }

    pub fn wrap_future<F: Future + Unpin>(&self, future: F) -> SessionedFuture<F> {
        let session_watcher = self.watch();
        SessionedFuture {
            session_watcher,
            future,
        }
    }

    pub fn wrap_stream<S: Stream + Unpin>(&self, stream: S) -> SessionedStream<S> {
        let session_watcher = self.watch();
        SessionedStream {
            session_watcher,
            stream,
        }
    }
}

pub struct SessionedFuture<F: Future> {
    session_watcher: SessionWatcher,
    future: F,
}

impl<F: Future + Unpin> Future for SessionedFuture<F> {
    type Output = Option<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.session_watcher.poll(cx) {
            Poll::Ready(()) => Poll::Ready(None),
            Poll::Pending => match self.future.poll(cx) {
                Poll::Ready(output) => Poll::Ready(Some(output)),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

pub struct SessionedStream<S: Stream> {
    session_watcher: SessionWatcher,
    stream: S,
}

impl<S: Stream + Unpin> Stream for SessionedStream<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.session_watcher.poll(cx) {
            Poll::Ready(()) => Poll::Ready(None),
            Poll::Pending => self.stream.poll_next(cx),
        }
    }
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        for (_, watcher_inner) in &self.watchers {
            let watcher_inner = watcher_inner
                .upgrade()
                .expect_throw("should successfuly upgraded");

            let SessionWatcherInner { ref mut waker } = *watcher_inner.borrow_mut();

            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }
}

impl Drop for SessionWatcher {
    fn drop(&mut self) {
        if let Some(session_inner) = self.session_inner.upgrade() {
            session_inner.borrow_mut().watchers.remove(self.index);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test() {
        let ready = std::future::ready(1);
        let pending = std::future::pending::<i32>();
        let stream = futures_lite::stream::repeat(1);

        let session = Session::new();
        let ready = session.wrap_future(ready);
        let pending = session.wrap_future(pending);
        let mut stream = session.wrap_stream(stream);

        assert_eq!(ready.await, Some(1));
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(1));
        drop(session);
        assert_eq!(pending.await, None);
        assert_eq!(stream.next().await, None);
    }
}
