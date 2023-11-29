use super::Node;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

impl<T> AsRef<web_sys::Element> for Node<T>
where
    T: AsRef<web_sys::Element>,
{
    #[inline]
    fn as_ref(&self) -> &web_sys::Element {
        self.0.as_ref()
    }
}

impl<E> Node<E>
where
    E: AsRef<web_sys::Element>,
{
    #[inline]
    pub fn remove(&self) {
        self.as_ref().remove();
    }

    #[inline]
    pub fn id(self, id: impl Into<Cow<'static, str>>) -> Self {
        self.as_ref().set_id(&id.into());
        self
    }

    #[inline]
    pub fn set_id(&self, id: impl Into<Cow<'static, str>>) -> &Self {
        self.as_ref().set_id(&id.into());
        self
    }

    #[inline]
    #[track_caller]
    pub fn class(self, class: impl Into<Cow<'static, str>>) -> Self {
        self.add_class(class);
        self
    }

    #[inline]
    #[track_caller]
    pub fn add_class(&self, class: impl Into<Cow<'static, str>>) -> &Self {
        self.as_ref()
            .class_list()
            .add_1(&class.into())
            .unwrap_throw();
        self
    }

    #[inline]
    #[track_caller]
    pub fn remove_class(&self, class: impl Into<Cow<'static, str>>) -> &Self {
        self.as_ref()
            .class_list()
            .remove_1(&class.into())
            .unwrap_throw();
        self
    }

    #[inline]
    #[track_caller]
    pub fn toggle_class(&self, class: impl Into<Cow<'static, str>>) -> bool {
        self.as_ref()
            .class_list()
            .toggle(&class.into())
            .unwrap_throw()
    }

    #[inline]
    #[track_caller]
    pub fn attr(
        self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.as_ref()
            .set_attribute(&name.into(), &value.into())
            .unwrap_throw();
        self
    }

    #[inline]
    #[track_caller]
    pub fn set_attr(
        &self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> &Self {
        self.as_ref()
            .set_attribute(&name.into(), &value.into())
            .unwrap_throw();
        self
    }

    #[inline]
    #[track_caller]
    pub fn children(&self) -> Vec<Node<web_sys::Element>> {
        let collection = self.as_ref().children();
        let len = collection.length();
        (0..len)
            .map(|i| Node(collection.item(i).unwrap_throw()))
            .collect()
    }

    #[inline]
    #[track_caller]
    pub fn query_selector(&self, selector: &str) -> Vec<Node<web_sys::Node>> {
        let node_list = self.as_ref().query_selector_all(selector).unwrap_throw();
        let len = node_list.length();
        (0..len)
            .map(|i| Node(node_list.item(i).unwrap_throw()))
            .collect()
    }
}
