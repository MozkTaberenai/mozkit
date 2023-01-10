use crate::*;

impl<T: IsEventTarget> Dom<T> {
    /// short hand of self.listen().into_stream()
    pub fn on<E: DomEvent>(&self) -> DomEventStream<E> {
        self.listen().into_stream()
        // DomEventStream::new(self.as_event_target().clone(), false, false)
    }

    // pub fn event_stream<E: DomEvent>(&self, capture: bool, passive: bool) -> DomEventStream<E> {
    //     DomEventStream::new(self.as_event_target().clone(), capture, passive)
    // }

    pub fn listen<E: DomEvent>(&self) -> DomEventListenerBuilder<E> {
        DomEventListenerBuilder::new(self.as_event_target().clone())
    }
}
