use super::{unchecked_create_element, Dom};

pub fn link() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("link")
}

pub fn style() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("style")
}

pub fn main() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("main")
}

pub fn header() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("header")
}

pub fn footer() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("footer")
}

pub fn aside() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("aside")
}

pub fn nav() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("nav")
}

pub fn section() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("section")
}

pub fn h1() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("h1")
}

pub fn h2() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("h2")
}

pub fn h3() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("h3")
}

pub fn div() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("div")
}

pub fn p() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("p")
}

pub fn span() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("span")
}

pub fn ul() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("ul")
}

pub fn ol() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("ol")
}

pub fn li() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("li")
}

pub fn i() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("i")
}

pub fn a() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("a")
}

#[cfg(feature = "html-button")]
pub fn button() -> Dom<web_sys::HtmlButtonElement> {
    unchecked_create_element("button")
}
#[cfg(not(feature = "html-button"))]
pub fn button() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("button")
}

#[cfg(feature = "html-input")]
pub fn input() -> Dom<web_sys::HtmlInputElement> {
    unchecked_create_element("input")
}
#[cfg(not(feature = "html-input"))]
pub fn input() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("input")
}

#[cfg(feature = "html-dialog")]
pub fn dialog() -> Dom<web_sys::HtmlDialogElement> {
    unchecked_create_element("dialog")
}
#[cfg(not(feature = "html-dialog"))]
pub fn dialog() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("dialog")
}
