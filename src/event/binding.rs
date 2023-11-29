use super::DomEvent;
use wasm_bindgen::prelude::*;

pub struct DomEventBinding<E: DomEvent> {
    pub(crate) target: web_sys::EventTarget,
    pub(crate) closure: Closure<dyn FnMut(web_sys::Event)>,
    pub(crate) capture: bool,
    pub(crate) _marker: std::marker::PhantomData<E>,
}

impl<E: DomEvent> Drop for DomEventBinding<E> {
    #[inline]
    fn drop(&mut self) {
        // only capture flag affect on remove event listener.
        // https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener#matching_event_listeners_for_removal
        self.target
            .remove_event_listener_with_callback_and_bool(
                E::TYPE_STR,
                self.closure.as_ref().unchecked_ref(),
                self.capture,
            )
            .unwrap_throw();
    }
}
