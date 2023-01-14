use crate::*;

pub trait IsEventTarget: JsCast {
    fn as_event_target(&self) -> &web_sys::EventTarget {
        self.unchecked_ref()
    }
    fn into_event_target(self) -> web_sys::EventTarget {
        self.unchecked_into()
    }
}
impl IsEventTarget for web_sys::EventTarget {}
impl IsEventTarget for web_sys::Window {}

impl<T: IsNode> IsEventTarget for T {}

pub trait IsNode: JsCast {
    fn as_node(&self) -> &web_sys::Node {
        self.unchecked_ref()
    }
    fn into_node(self) -> web_sys::Node {
        self.unchecked_into()
    }
}
impl IsNode for web_sys::Node {}
impl IsNode for web_sys::Document {}
impl IsNode for web_sys::DocumentFragment {}
impl IsNode for web_sys::Text {}

impl<T: IsElement> IsNode for T {}

pub trait IsElement: JsCast {
    fn as_element(&self) -> &web_sys::Element {
        self.unchecked_ref()
    }
    fn into_element(self) -> web_sys::Element {
        self.unchecked_into()
    }
}
impl IsElement for web_sys::Element {}
impl IsElement for web_sys::HtmlElement {}
impl IsElement for web_sys::HtmlHeadElement {}
impl IsElement for web_sys::SvgElement {}

#[cfg(feature = "html-input")]
impl IsElement for web_sys::HtmlInputElement {}
#[cfg(feature = "html-dialog")]
impl IsElement for web_sys::HtmlDialogElement {}
#[cfg(feature = "html-button")]
impl IsElement for web_sys::HtmlButtonElement {}
#[cfg(feature = "html-link")]
impl IsElement for web_sys::HtmlLinkElement {}
#[cfg(feature = "html-style")]
impl IsElement for web_sys::HtmlStyleElement {}
#[cfg(feature = "html-div")]
impl IsElement for web_sys::HtmlDivElement {}
#[cfg(feature = "html-p")]
impl IsElement for web_sys::HtmlParagraphElement {}

#[cfg(feature = "svg-svg")]
impl IsElement for web_sys::SvgsvgElement {}

#[cfg(feature = "media-query-list")]
impl IsEventTarget for web_sys::MediaQueryList {}
