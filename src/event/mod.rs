mod binding;
mod builder;
mod stream;

pub use binding::DomEventBinding;
pub use builder::DomEventListenerBuilder;
pub use stream::DomEventStream;

use crate::StreamSpawnExt;

pub trait EventTargetExt {
    fn listen<E: DomEvent>(&self) -> DomEventListenerBuilder<E>;

    /// create event listener stream and spawn it.
    /// shorthand of event_target.listen::<E>().into_stream().for_each_spawn(handler_fn)
    #[inline]
    fn add_listener<E: DomEvent>(&self, f: impl FnMut(web_sys::Event) + 'static) {
        self.listen::<E>().into_stream().for_each_spawn(f)
    }
}

impl<T: AsRef<web_sys::EventTarget>> EventTargetExt for T {
    #[inline]
    fn listen<E: DomEvent>(&self) -> DomEventListenerBuilder<E> {
        DomEventListenerBuilder::new(self.as_ref().clone())
    }
}

pub trait DomEvent: Unpin + 'static {
    const TYPE_STR: &'static str;
}

macro_rules! impl_dom_event {
    ($t:ident, $s:expr) => {
        pub struct $t;
        impl DomEvent for $t {
            const TYPE_STR: &'static str = $s;
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
