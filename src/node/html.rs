use super::Node;
use crate::document;
use wasm_bindgen::prelude::*;

#[inline]
fn create_prototype(tag: &'static str) -> web_sys::Element {
    document().create_element(tag).unwrap_throw()
}

macro_rules! impl_create_html {
    ($fn:ident, $tag:expr) => {
        #[inline]
        pub fn $fn() -> Node<web_sys::HtmlElement> {
            thread_local! {
                static PROTOTYPE: web_sys::Element = create_prototype($tag);
            }
            Node(PROTOTYPE.with(|elem| elem.clone_node().unwrap_throw().unchecked_into()))
        }
    };
}

impl_create_html!(link, "link");
impl_create_html!(style, "style");
impl_create_html!(script, "script");
impl_create_html!(main, "main");
impl_create_html!(header, "header");
impl_create_html!(footer, "footer");
impl_create_html!(aside, "aside");
impl_create_html!(nav, "nav");
impl_create_html!(section, "section");
impl_create_html!(h1, "h1");
impl_create_html!(h2, "h2");
impl_create_html!(h3, "h3");
impl_create_html!(h4, "h4");
impl_create_html!(h5, "h5");
impl_create_html!(h6, "h6");
impl_create_html!(div, "div");
impl_create_html!(p, "p");
impl_create_html!(span, "span");
impl_create_html!(i, "i");
impl_create_html!(ins, "ins");
impl_create_html!(del, "del");
impl_create_html!(a, "a");
impl_create_html!(ul, "ul");
impl_create_html!(ol, "ol");
impl_create_html!(li, "li");
impl_create_html!(input, "input");
impl_create_html!(button, "button");
impl_create_html!(select, "select");
impl_create_html!(option, "option");
impl_create_html!(optgroup, "optgroup");
impl_create_html!(dialog, "dialog");
