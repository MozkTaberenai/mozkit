use super::*;
use wasm_bindgen::prelude::*;

pub struct DomEventListenerBuilder<E: DomEvent> {
    target: web_sys::EventTarget,
    _marker: std::marker::PhantomData<E>,
    capture: bool,
    passive: Option<bool>,
    prevent_default: bool,
    stop_propagation: bool,
    stop_immediate_propagation: bool,
}

impl<E: DomEvent> DomEventListenerBuilder<E> {
    #[inline]
    pub(crate) fn new(target: web_sys::EventTarget) -> Self {
        Self {
            target,
            _marker: std::marker::PhantomData,
            capture: false,
            passive: None,
            prevent_default: false,
            stop_propagation: false,
            stop_immediate_propagation: false,
        }
    }

    #[inline]
    pub fn capture(mut self) -> Self {
        self.capture = true;
        self
    }

    #[inline]
    pub fn passive(mut self, value: bool) -> Self {
        self.passive.replace(value);
        self
    }

    #[inline]
    pub fn prevent_default(mut self) -> Self {
        self.prevent_default = true;
        self
    }

    #[inline]
    pub fn stop_propagation(mut self) -> Self {
        self.stop_propagation = true;
        self
    }

    #[inline]
    pub fn stop_immediate_propagation(mut self) -> Self {
        self.stop_immediate_propagation = true;
        self
    }

    #[inline]
    pub fn callback(
        self,
        mut callback: impl FnMut(E::WebSysEvent) + 'static,
    ) -> DomEventBinding<E> {
        let Self {
            target,
            capture,
            passive,
            prevent_default,
            stop_propagation,
            stop_immediate_propagation,
            ..
        } = self;

        let closure = {
            Closure::new(move |event: web_sys::Event| {
                if prevent_default {
                    event.prevent_default();
                }
                if stop_propagation {
                    event.stop_propagation();
                }
                if stop_immediate_propagation {
                    event.stop_immediate_propagation();
                }
                match event.dyn_into::<E::WebSysEvent>() {
                    Ok(event) => callback(event),
                    Err(_) => crate::error!(
                        "fail to JsCast::dyn_into::<{}>()",
                        std::any::type_name::<E::WebSysEvent>()
                    ),
                };
            })
        };

        let options = web_sys::AddEventListenerOptions::new();
        options.set_capture(capture);
        if let Some(passive) = passive {
            options.set_passive(passive);
        }

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                E::TYPE_STR,
                closure.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        DomEventBinding {
            target,
            closure,
            capture,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn into_stream(self) -> DomEventStream<E> {
        let (tx, rx) = crate::chan(16);
        let _binding = self.callback(move |event| {
            if let Err(err) = tx.send(event) {
                crate::error!("fail to send: {}", err.to_string());
            }
        });

        DomEventStream { _binding, rx }
    }
}
