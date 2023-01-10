use super::{unchecked_create_element_ns, Dom};

pub const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

#[cfg(feature = "svg-svg")]
pub fn svg() -> Dom<web_sys::SvgsvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "svg")
}
#[cfg(not(feature = "svg-svg"))]
pub fn svg() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "svg")
}

pub fn path() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "path")
}

pub fn g() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "g")
}

pub fn circle() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "circle")
}

pub fn rect() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "rect")
}

pub fn text() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "text")
}

pub fn symbol() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "symbol")
}

pub fn use_() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "use")
}

pub fn defs() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "defs")
}

pub fn linear_gradient() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "linearGradient")
}

pub fn radial_gradient() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "radialGradient")
}

pub fn stop() -> Dom<web_sys::SvgElement> {
    unchecked_create_element_ns(SVG_NAMESPACE_URI, "stop")
}
