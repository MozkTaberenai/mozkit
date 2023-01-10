use crate::*;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
// use futures_lite::stream::ForEachFuture;
// use futures_util::stream::ForEach;

pub trait StreamExt: Stream {
    fn next(&mut self) -> Next<'_, Self> {
        Next { stream: self }
    }

    fn for_each<E, F>(self, mut f: E) -> F
    where
        Self: Sized + Unpin,
        E: FnMut(Self::Item),
        F: Future<Output = ()>,
    {
        async move {
            while let Some(item) = self.next().await {
                f(item);
            }
        }
    }
}
impl<S> StreamExt for S where S: Stream {}

pub struct Next<'a, S: ?Sized> {
    stream: &'a mut S,
}

impl<S: Unpin + ?Sized> Unpin for Next<'_, S> {}

impl<S: Stream + Unpin + ?Sized> Future for Next<'_, S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream.poll_next(cx)
    }
}

// pub trait ForEachSync: Stream {
//     fn for_each_sync<F, F2, Fut>(self, mut f: F) -> ForEach<Self, Fut, F2>
//     where
//         Self: Sized,
//         F: FnMut(Self::Item),
//         F2: FnMut(Self::Item) -> Fut,
//         Fut: Future<Output = ()>,
//     {
//         self.for_each(move |item| {
//             f(item);
//             std::future::ready(())
//         } as F2)
//     }
// }

// type BoxFnMut<T> = Box<dyn FnMut(T)>;

// pub trait ForEachLogErrExt: Stream {
//     fn for_each_log_err<E, F, Fut, F2, Fut2>(self, mut f: F) -> ForEach<Self, Fut2, F2>
//     where
//         Self: Sized,
//         E: std::error::Error,
//         F: FnMut(Self::Item) -> Fut,
//         Fut: Future<Output = Result<(), E>>,
//         F2: FnMut(Self::Item) -> Fut2,
//         Fut2: Future<Output = ()>,
//     {
//         self.for_each(move |item| async move {
//             if let Err(err) = f(item).await {
//                 error!("{err}");
//             }
//         })
//     }
// }

// impl<S> ForEachLogErrExt for S where S: Stream {}
