use log::error;
use std::future::Future;

pub fn spawn(f: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(f)
}

fn spawn_log_err<F, E>(f: F)
where
    F: Future<Output = Result<(), E>> + 'static,
    E: std::error::Error,
{
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(err) = f.await {
            error!("{err}");
        }
    })
}

pub trait FutureSpawnExt: Future<Output = ()> + Sized + 'static {
    fn spawn(self) {
        spawn(self);
    }
}
impl<F> FutureSpawnExt for F where F: Future<Output = ()> + Sized + 'static {}

pub trait FutureSpawnLogErrExt<E>: Future<Output = Result<(), E>> + Sized + 'static
where
    E: std::error::Error,
{
    fn spawn(self) {
        spawn_log_err(self)
    }
}
impl<F, E> FutureSpawnLogErrExt<E> for F
where
    F: Future<Output = Result<(), E>> + Sized + 'static,
    E: std::error::Error,
{
}
