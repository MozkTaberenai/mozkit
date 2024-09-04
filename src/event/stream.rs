use super::{DomEvent, DomEventBinding};
use crate::chan;
use futures_lite::prelude::*;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct DomEventStream<E: DomEvent> {
    pub(crate) _binding: DomEventBinding<E>,
    pub(crate) rx: chan::Receiver<E::WebSysEvent>,
}

impl<E: DomEvent> Stream for DomEventStream<E> {
    type Item = E::WebSysEvent;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next(cx)
    }
}
