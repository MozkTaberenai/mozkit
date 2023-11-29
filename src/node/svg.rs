use super::Node;
use crate::document;
use wasm_bindgen::prelude::*;

#[inline]
fn create_prototype(tag: &'static str) -> web_sys::Element {
    document()
        .create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        .unwrap_throw()
}

macro_rules! impl_create_svg {
    ($fn:ident, $tag:expr) => {
        #[inline]
        pub fn $fn() -> Node<web_sys::SvgElement> {
            thread_local! {
                static PROTOTYPE: web_sys::Element = create_prototype($tag);
            }
            Node(PROTOTYPE.with(|elem| elem.clone_node().unwrap_throw().unchecked_into()))
        }
    };
}

impl_create_svg!(svg, "svg");
impl_create_svg!(path, "path");
impl_create_svg!(g, "g");
impl_create_svg!(circle, "circle");
impl_create_svg!(rect, "rect");
impl_create_svg!(text, "text");
impl_create_svg!(symbol, "symbol");
impl_create_svg!(use_, "use");
impl_create_svg!(defs, "defs");
impl_create_svg!(linear_gradient, "linearGradient");
impl_create_svg!(radial_gradient, "radialGradient");
impl_create_svg!(stop, "stop");
