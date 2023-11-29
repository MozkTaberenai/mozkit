use super::Node;
use crate::Storage;
use wasm_bindgen::prelude::*;

thread_local! {
    static WINDOW: web_sys::Window = web_sys::window().unwrap_throw();
    static DOCUMENT: web_sys::Document = window().document().unwrap_throw();
    static HISTORY: web_sys::History = window().history().unwrap_throw();
    static LOCAL_STORAGE: Storage = Storage(window().local_storage().unwrap_throw().unwrap_throw());
    static SESSION_STORAGE: Storage = Storage(window().session_storage().unwrap_throw().unwrap_throw());
    static DOCUMENT_ELEMENT: Node<web_sys::Element> = Node(document().document_element().unwrap_throw());
    static HEAD: Node<web_sys::HtmlHeadElement> = Node(document().head().unwrap_throw().unchecked_into());
    static BODY: Node<web_sys::HtmlElement> = Node(document().body().unwrap_throw().unchecked_into());
}

#[inline]
pub fn window() -> web_sys::Window {
    WINDOW.with(Clone::clone)
}

#[inline]
pub fn document() -> web_sys::Document {
    DOCUMENT.with(Clone::clone)
}

#[inline]
pub fn history() -> web_sys::History {
    HISTORY.with(Clone::clone)
}

#[inline]
pub fn local_storage() -> Storage {
    LOCAL_STORAGE.with(Clone::clone)
}

#[inline]
pub fn session_storage() -> Storage {
    SESSION_STORAGE.with(Clone::clone)
}

#[inline]
pub fn document_element() -> Node<web_sys::Element> {
    DOCUMENT_ELEMENT.with(Clone::clone)
}

#[inline]
pub fn head() -> Node<web_sys::HtmlHeadElement> {
    HEAD.with(Clone::clone)
}

#[inline]
pub fn body() -> Node<web_sys::HtmlElement> {
    BODY.with(Clone::clone)
}
