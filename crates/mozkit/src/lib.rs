pub use futures_lite::prelude::*;
// pub use futures_core::Stream;
// pub use futures_util::StreamExt;
pub use log::{debug, error, info, trace, warn};
pub use std::future::Future;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
pub use wasm_bindgen_futures::JsFuture;
pub use web_sys::{DocumentFragment, Element, HtmlElement, Node, SvgElement};

#[cfg(feature = "html-button")]
pub use web_sys::HtmlButtonElement;
#[cfg(feature = "html-dialog")]
pub use web_sys::HtmlDialogElement;
#[cfg(feature = "html-div")]
pub use web_sys::HtmlDivElement;
#[cfg(feature = "html-input")]
pub use web_sys::HtmlInputElement;
#[cfg(feature = "html-link")]
pub use web_sys::HtmlLinkElement;
#[cfg(feature = "html-p")]
pub use web_sys::HtmlParagraphElement;
#[cfg(feature = "html-style")]
pub use web_sys::HtmlStyleElement;
#[cfg(feature = "media-query-list")]
pub use web_sys::MediaQueryList;
#[cfg(feature = "svg-svg")]
pub use web_sys::SvgsvgElement;

mod rt;
pub use rt::*;

pub fn init_logger(level: log::Level) {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(level));
}

mod chan;
pub use chan::*;
mod event;
pub use event::*;
mod session;
pub use session::*;
mod state;
pub use state::*;

mod dom;
pub use dom::*;
mod get;
pub use get::*;
mod timer;
pub use timer::{Interval, Timeout};
mod storage;
pub use storage::Storage;
pub mod fetch;

mod error;
pub use error::*;
