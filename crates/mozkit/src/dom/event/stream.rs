use crate::*;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct DomEventStream<E: DomEvent> {
    pub binding: DomEventBinding<E>,
    pub stream: ChannelRx<E::WebSysEvent>,
}

impl<E: DomEvent> Stream for DomEventStream<E> {
    type Item = E::WebSysEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.poll_next(cx)
    }
}
