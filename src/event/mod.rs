mod binding;
mod builder;
mod stream;

pub use binding::DomEventBinding;
pub use builder::DomEventListenerBuilder;
pub use stream::DomEventStream;

use crate::StreamSpawnExt;
use wasm_bindgen::JsCast;

pub trait EventTargetExt {
    fn make_listener<E: DomEvent>(&self) -> DomEventListenerBuilder<E>;

    /// create event listener stream and spawn it.
    /// shorthand of event_target.make_listener::<E>().into_stream().for_each_spawn(f)
    #[inline]
    fn listen<E: DomEvent>(&self, f: impl FnMut(E::WebSysEvent) + 'static) {
        self.make_listener::<E>().into_stream().for_each_spawn(f)
    }
}

impl<T: AsRef<web_sys::EventTarget>> EventTargetExt for T {
    #[inline]
    fn make_listener<E: DomEvent>(&self) -> DomEventListenerBuilder<E> {
        DomEventListenerBuilder::new(self.as_ref().clone())
    }
}

pub trait DomEvent: Unpin + 'static {
    const TYPE_STR: &'static str;
    type WebSysEvent: JsCast + 'static;
}

macro_rules! impl_dom_event {
    ($t:ident, $s:expr) => {
        pub struct $t<T = web_sys::Event>(std::marker::PhantomData<T>);
        impl<T: JsCast + Unpin + 'static> DomEvent for $t<T> {
            const TYPE_STR: &'static str = $s;
            type WebSysEvent = T;
        }
    };
}

impl_dom_event!(ClickEvent, "click");
impl_dom_event!(KeyDownEvent, "keydown");
impl_dom_event!(KeyUpEvent, "keyup");
impl_dom_event!(ScrollEvent, "scroll");
impl_dom_event!(ResizeEvent, "resize");
impl_dom_event!(PopStateEvent, "popstate");
impl_dom_event!(AnimationStartEvent, "animationstart");
impl_dom_event!(AnimationEndEvent, "animationend");
impl_dom_event!(AnimationIterationEvent, "animationiteration");
impl_dom_event!(AnimationCancelEvent, "animationcancel");
impl_dom_event!(ChangeEvent, "change");
impl_dom_event!(ConnectEvent, "connect");
impl_dom_event!(MessageEvent, "message");
impl_dom_event!(ErrorEvent, "error");
impl_dom_event!(MessageErrorEvent, "messageerror");
impl_dom_event!(InstallEvent, "install");
impl_dom_event!(ActivateEvent, "activate");
impl_dom_event!(FetchEvent, "fetch");
impl_dom_event!(PushEvent, "push");
