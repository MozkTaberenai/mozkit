use crate::*;

impl<T: IsEventTarget> Dom<T> {
    /// short hand of self.listen().into_stream()
    pub fn on<E: DomEvent>(&self) -> DomEventStream<E> {
        self.listen().into_stream()
    }

    pub fn listen<E: DomEvent>(&self) -> DomEventListenerBuilder<E> {
        DomEventListenerBuilder::new(self.as_event_target().clone())
    }
}
