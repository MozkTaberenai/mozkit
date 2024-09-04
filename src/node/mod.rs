use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T>(pub T);

impl<T> std::ops::Deref for Node<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<web_sys::EventTarget> for Node<T>
where
    T: AsRef<web_sys::EventTarget>,
{
    #[inline]
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.0.as_ref()
    }
}

impl<T> AsRef<web_sys::Node> for Node<T>
where
    T: AsRef<web_sys::Node>,
{
    #[inline]
    fn as_ref(&self) -> &web_sys::Node {
        self.0.as_ref()
    }
}

impl<F: JsCast> Node<F> {
    #[inline]
    pub fn dyn_into<T: JsCast>(self) -> Result<Node<T>, Self> {
        self.0.dyn_into::<T>().map(Node).map_err(Node)
    }

    #[inline]
    pub fn unchecked_into<T: JsCast>(self) -> Node<T> {
        Node(self.0.unchecked_into())
    }
}

use crate::event::{DomEvent, EventTargetExt};

impl<T> Node<T>
where
    T: AsRef<web_sys::EventTarget>,
{
    #[inline]
    pub fn on<E: DomEvent>(self, f: impl FnMut(E::WebSysEvent) + 'static) -> Self {
        self.listen::<E>(f);
        self
    }
}

impl<T> Node<T>
where
    T: AsRef<web_sys::Node>,
{
    #[inline]
    #[track_caller]
    pub fn child(self, child: impl AsRef<web_sys::Node>) -> Self {
        self.as_ref().append_child(child.as_ref()).unwrap_throw();
        self
    }

    #[inline]
    #[track_caller]
    pub fn append(&self, child: impl AsRef<web_sys::Node>) -> &Self {
        self.as_ref().append_child(child.as_ref()).unwrap_throw();
        self
    }
}

#[inline]
pub fn text_node(data: &str) -> Node<web_sys::Text> {
    Node(crate::document().create_text_node(data))
}

mod element;

pub mod html;
pub mod svg;
