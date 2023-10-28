mod binding;
mod builder;
mod stream;

pub use binding::DomEventBinding;
pub use builder::DomEventListenerBuilder;
pub use stream::DomEventStream;

pub trait DomEvent {
    const NAME: &'static str;
    type WebSysEvent: AsRef<web_sys::Event> + wasm_bindgen::convert::FromWasmAbi + 'static;
}

pub struct ClickEvent;
impl DomEvent for ClickEvent {
    const NAME: &'static str = "click";
    type WebSysEvent = web_sys::MouseEvent;
}

pub struct KeyDownEvent;
impl DomEvent for KeyDownEvent {
    const NAME: &'static str = "keydown";
    type WebSysEvent = web_sys::KeyboardEvent;
}

pub struct KeyUpEvent;
impl DomEvent for KeyUpEvent {
    const NAME: &'static str = "keyup";
    type WebSysEvent = web_sys::KeyboardEvent;
}

pub struct ScrollEvent;
impl DomEvent for ScrollEvent {
    const NAME: &'static str = "scroll";
    type WebSysEvent = web_sys::Event;
}

pub struct ResizeEvent;
impl DomEvent for ResizeEvent {
    const NAME: &'static str = "resize";
    type WebSysEvent = web_sys::UiEvent;
}

pub struct PopStateEvent;
impl DomEvent for PopStateEvent {
    const NAME: &'static str = "popstate";
    #[cfg(feature = "event-popstate")]
    type WebSysEvent = web_sys::PopStateEvent;
    #[cfg(not(feature = "event-popstate"))]
    type WebSysEvent = web_sys::Event;
}

#[cfg(feature = "event-animation")]
mod animation {
    use super::*;

    pub struct AnimationStartEvent;
    impl DomEvent for AnimationStartEvent {
        const NAME: &'static str = "animationstart";
        type WebSysEvent = web_sys::AnimationEvent;
    }
    pub struct AnimationEndEvent;
    impl DomEvent for AnimationEndEvent {
        const NAME: &'static str = "animationend";
        type WebSysEvent = web_sys::AnimationEvent;
    }
    pub struct AnimationIterationEvent;
    impl DomEvent for AnimationIterationEvent {
        const NAME: &'static str = "animationiteration";
        type WebSysEvent = web_sys::AnimationEvent;
    }
    pub struct AnimationCancelEvent;
    impl DomEvent for AnimationCancelEvent {
        const NAME: &'static str = "animationcancel";
        type WebSysEvent = web_sys::AnimationEvent;
    }
}
#[cfg(feature = "event-animation")]
pub use animation::*;

#[cfg(feature = "media-query-list")]
pub struct MediaQueryListChangeEvent;
#[cfg(feature = "media-query-list")]
impl DomEvent for MediaQueryListChangeEvent {
    const NAME: &'static str = "change";
    type WebSysEvent = web_sys::MediaQueryListEvent;
}
