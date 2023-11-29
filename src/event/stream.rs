use super::{DomEvent, DomEventBinding};
use crate::chan;
use futures_lite::prelude::*;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct DomEventStream<E: DomEvent> {
    pub(crate) _binding: DomEventBinding<E>,
    pub(crate) rx: chan::Receiver<web_sys::Event>,
}

impl<E: DomEvent> Stream for DomEventStream<E> {
    type Item = web_sys::Event;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next(cx)
    }
}
