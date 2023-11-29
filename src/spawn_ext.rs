use futures_lite::prelude::*;

pub trait FutureSpawnExt: Future<Output = ()> + Sized + 'static {
    #[inline]
    fn spawn(self) {
        wasm_bindgen_futures::spawn_local(self);
    }
}

impl<F> FutureSpawnExt for F where F: Future<Output = ()> + Sized + 'static {}

pub trait FutureSpawnLogErrExt<E>: Future<Output = Result<(), E>> + Sized + 'static
where
    E: std::fmt::Display,
{
    #[inline]
    fn spawn(self) {
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(err) = self.await {
                crate::error!("{err}");
            }
        });
    }
}

impl<F, E> FutureSpawnLogErrExt<E> for F
where
    F: Future<Output = Result<(), E>> + Sized + 'static,
    E: std::fmt::Display,
{
}

pub trait StreamSpawnExt: Stream + Sized + 'static {
    #[inline]
    fn for_each_spawn<F>(self, f: F)
    where
        F: FnMut(<Self as Stream>::Item) + 'static,
    {
        futures_lite::StreamExt::for_each(self, f).spawn();
    }
}

impl<S> StreamSpawnExt for S where S: Stream + Sized + 'static {}
