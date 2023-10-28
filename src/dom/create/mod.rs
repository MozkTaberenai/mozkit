use crate::*;

pub fn text_node(data: &str) -> Dom<web_sys::Text> {
    document().create_text_node(data).into()
}

pub fn docfrag() -> Dom<web_sys::DocumentFragment> {
    document().create_document_fragment().into()
}

pub fn create_element<T: JsCast>(name: &str) -> Dom<T> {
    let inner = document()
        .create_element(name)
        .unwrap_js()
        .dyn_into::<T>()
        .expect_throw("JsCast::dyn_into() failed");
    Dom { inner }
}

pub(crate) fn unchecked_create_element<T: JsCast>(name: &str) -> Dom<T> {
    let inner = document()
        .create_element(name)
        .unwrap_js()
        .unchecked_into::<T>();
    Dom { inner }
}

pub fn create_element_ns<T: JsCast>(namespace: &str, name: &str) -> Dom<T> {
    let inner = document()
        .create_element_ns(Some(namespace), name)
        .unwrap_js()
        .dyn_into::<T>()
        .expect_throw("JsCast::dyn_into() failed");
    Dom { inner }
}

pub(crate) fn unchecked_create_element_ns<T: JsCast>(namespace: &str, name: &str) -> Dom<T> {
    let inner = document()
        .create_element_ns(Some(namespace), name)
        .unwrap_js()
        .unchecked_into::<T>();
    Dom { inner }
}

pub mod html;
pub mod svg;
