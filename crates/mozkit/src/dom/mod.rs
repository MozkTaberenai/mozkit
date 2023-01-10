use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dom<T> {
    pub inner: T,
}

impl<T> From<T> for Dom<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> std::ops::Deref for Dom<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait DomJsCastExt<F> {
    fn checked_into<T: JsCast>(self) -> Option<Dom<T>>;
    fn unchecked_into<T: JsCast>(self) -> Dom<T>;
}

impl<F: JsCast> DomJsCastExt<F> for Dom<F> {
    fn checked_into<T: JsCast>(self) -> Option<Dom<T>> {
        match self.inner.dyn_into::<T>() {
            Ok(node) => Some(Dom::from(node)),
            Err(_) => None,
        }
    }
    fn unchecked_into<T: JsCast>(self) -> Dom<T> {
        Dom::from(self.inner.unchecked_into::<T>())
    }
}

mod create;
pub use create::*;
mod event;
pub use event::*;

mod as_;
pub use as_::*;

mod event_target;
pub use event_target::*;
mod node;
pub use node::*;
mod element;
pub use element::*;
