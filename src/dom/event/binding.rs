use crate::*;
use web_sys::EventTarget;

pub struct DomEventBinding<E: DomEvent> {
    pub(crate) target: EventTarget,
    pub(crate) closure: Closure<dyn FnMut(E::WebSysEvent)>,
    pub(crate) capture: bool,
}

impl<E: DomEvent> Drop for DomEventBinding<E> {
    fn drop(&mut self) {
        // only capture flag affect on remove event listener.
        // https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener#matching_event_listeners_for_removal
        self.target
            .remove_event_listener_with_callback_and_bool(
                E::NAME,
                self.closure.as_ref().unchecked_ref(),
                self.capture,
            )
            .unwrap_js();
    }
}
