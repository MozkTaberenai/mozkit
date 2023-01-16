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

pub fn i() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("i")
}

pub fn a() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("a")
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

#[cfg(feature = "html-input")]
pub fn input() -> Dom<web_sys::HtmlInputElement> {
    unchecked_create_element("input")
}
#[cfg(not(feature = "html-input"))]
pub fn input() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("input")
}

#[cfg(feature = "html-button")]
pub fn button() -> Dom<web_sys::HtmlButtonElement> {
    unchecked_create_element("button")
}
#[cfg(not(feature = "html-button"))]
pub fn button() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("button")
}

#[cfg(feature = "html-select")]
pub fn select() -> Dom<web_sys::HtmlSelectElement> {
    unchecked_create_element("select")
}
#[cfg(not(feature = "html-select"))]
pub fn select() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("select")
}

#[cfg(feature = "html-select")]
pub fn option() -> Dom<web_sys::HtmlOptionElement> {
    unchecked_create_element("option")
}
#[cfg(not(feature = "html-select"))]
pub fn option() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("option")
}

#[cfg(feature = "html-select")]
pub fn optgroup() -> Dom<web_sys::HtmlOptGroupElement> {
    unchecked_create_element("optgroup")
}
#[cfg(not(feature = "html-select"))]
pub fn optgroup() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("optgroup")
}

#[cfg(feature = "html-dialog")]
pub fn dialog() -> Dom<web_sys::HtmlDialogElement> {
    unchecked_create_element("dialog")
}
#[cfg(not(feature = "html-dialog"))]
pub fn dialog() -> Dom<web_sys::HtmlElement> {
    unchecked_create_element("dialog")
}
