use futures_lite::prelude::*;
use slab::Slab;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Scope {
    inner: Rc<RefCell<ScopeInner>>,
}

#[derive(Debug, Default)]
struct ScopeInner {
    wakers: Slab<Weak<RefCell<Option<Waker>>>>,
}

impl Drop for ScopeInner {
    #[inline]
    fn drop(&mut self) {
        for (_, waker) in &self.wakers {
            let waker = waker.upgrade().unwrap_throw();
            let mut waker = waker.borrow_mut();
            if let Some(waker) = waker.take() {
                waker.wake();
            }
        }
    }
}

#[derive(Debug)]
struct ScopeDropFuture {
    scope: Weak<RefCell<ScopeInner>>,
    waker: Rc<RefCell<Option<Waker>>>,
    waker_index: usize,
}

impl Future for ScopeDropFuture {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.scope.upgrade().is_some() {
            true => {
                self.waker.borrow_mut().replace(cx.waker().clone());
                Poll::Pending
            }
            false => Poll::Ready(()),
        }
    }
}

impl Drop for ScopeDropFuture {
    #[inline]
    fn drop(&mut self) {
        if let Some(scope_inner) = self.scope.upgrade() {
            scope_inner.borrow_mut().wakers.remove(self.waker_index);
        }
    }
}

impl Scope {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn make_scope_drop_future(&self) -> ScopeDropFuture {
        let scope = Rc::downgrade(&self.inner);
        let waker = Rc::new(RefCell::new(None));
        let waker_index = self.inner.borrow_mut().wakers.insert(Rc::downgrade(&waker));
        ScopeDropFuture {
            waker,
            scope,
            waker_index,
        }
    }

    #[inline]
    pub fn wrap_future<F: Future + Unpin>(&self, future: F) -> ScopedFuture<F> {
        let scope_drop_future = self.make_scope_drop_future();
        ScopedFuture {
            scope_drop_future,
            future,
        }
    }

    #[inline]
    pub fn wrap_stream<S: Stream + Unpin>(&self, stream: S) -> ScopedStream<S> {
        let scope_drop_future = self.make_scope_drop_future();
        ScopedStream {
            scope_drop_future,
            stream,
        }
    }
}

#[derive(Debug)]
pub struct ScopedFuture<F: Future> {
    scope_drop_future: ScopeDropFuture,
    future: F,
}

impl<F: Future + Unpin> Future for ScopedFuture<F> {
    type Output = Option<F::Output>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.scope_drop_future.poll(cx) {
            Poll::Ready(()) => Poll::Ready(None),
            Poll::Pending => match self.future.poll(cx) {
                Poll::Ready(output) => Poll::Ready(Some(output)),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

#[derive(Debug)]
pub struct ScopedStream<S: Stream> {
    scope_drop_future: ScopeDropFuture,
    stream: S,
}

impl<S: Stream + Unpin> Stream for ScopedStream<S> {
    type Item = S::Item;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.scope_drop_future.poll(cx) {
            Poll::Ready(()) => Poll::Ready(None),
            Poll::Pending => self.stream.poll_next(cx),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures_lite::StreamExt;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test() {
        let ready = std::future::ready(1);
        let pending = std::future::pending::<i32>();
        let stream = futures_lite::stream::repeat(1);

        let scope = Scope::new();
        let ready = scope.wrap_future(ready);
        let pending = scope.wrap_future(pending);
        let mut stream = scope.wrap_stream(stream);

        assert_eq!(ready.await, Some(1));
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(1));
        drop(scope);
        assert_eq!(pending.await, None);
        assert_eq!(stream.next().await, None);
    }
}
