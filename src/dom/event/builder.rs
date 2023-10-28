use crate::*;
use std::marker::PhantomData;
use web_sys::{AddEventListenerOptions, EventTarget};

pub struct DomEventListenerBuilder<E: DomEvent> {
    target: EventTarget,
    _marker: PhantomData<E>,
    capture: bool,
    passive: Option<bool>,
    prevent_default: bool,
    stop_propagation: bool,
    stop_immediate_propagation: bool,
}

impl<E: DomEvent> DomEventListenerBuilder<E> {
    pub fn new(target: EventTarget) -> Self {
        Self {
            target,
            _marker: PhantomData,
            capture: false,
            passive: None,
            prevent_default: false,
            stop_propagation: false,
            stop_immediate_propagation: false,
        }
    }

    pub fn capture(mut self) -> Self {
        self.capture = true;
        self
    }

    pub fn passive(mut self, value: bool) -> Self {
        self.passive.replace(value);
        self
    }

    pub fn prevent_default(mut self) -> Self {
        self.prevent_default = true;
        self
    }

    pub fn stop_propagation(mut self) -> Self {
        self.stop_propagation = true;
        self
    }

    pub fn stop_immediate_propagation(mut self) -> Self {
        self.stop_immediate_propagation = true;
        self
    }

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
            Closure::<dyn FnMut(E::WebSysEvent)>::new(move |event: E::WebSysEvent| {
                let event_ref = event.as_ref();
                if prevent_default {
                    event_ref.prevent_default();
                }
                if stop_propagation {
                    event_ref.stop_propagation();
                }
                if stop_immediate_propagation {
                    event_ref.stop_immediate_propagation();
                }
                callback(event);
            })
        };

        let mut options = AddEventListenerOptions::new();
        options.capture(capture);
        if let Some(passive) = passive {
            options.passive(passive);
        }

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                E::NAME,
                closure.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_js();

        DomEventBinding {
            target,
            closure,
            capture,
        }
    }

    pub fn into_stream(self) -> DomEventStream<E> {
        let (tx, rx) = Channel::new(256).split_into();

        let binding = self.callback(move |event| {
            if let Err(err) = tx.send(event) {
                match err {
                    ChannelSendError::Closed(event) => error!(
                        "DomEventBinding: AsyncQueue Rx closed: {}",
                        event.as_ref().to_string()
                    ),
                    ChannelSendError::Full(event) => error!(
                        "DomEventBinding: AsyncQueue Rx no available capacity: {}",
                        event.as_ref().to_string()
                    ),
                }
            }
        });

        DomEventStream {
            binding,
            stream: rx,
        }
    }
}
