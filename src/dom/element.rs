use super::*;

pub trait DomElementExt<T> {
    fn set_id(&self, id: &str);
    fn add_class(&self, class: &str);
    fn remove_class(&self, class: &str);
    fn toggle_class(&self, class: &str) -> bool;
    fn set_attr(&self, name: &str, value: &str);
    fn remove(&self);
    fn children(&self) -> Vec<Dom<Element>>;
    fn query_selector(&self, selector: &str) -> Vec<Dom<Node>>;
}

impl<T: IsElement> DomElementExt<T> for Dom<T> {
    fn set_id(&self, id: &str) {
        self.as_element().set_id(id);
    }

    #[track_caller]
    fn add_class(&self, class: &str) {
        self.as_element().class_list().add_1(class).unwrap_js()
    }

    #[track_caller]
    fn remove_class(&self, class: &str) {
        self.as_element().class_list().remove_1(class).unwrap_js()
    }

    #[track_caller]
    fn toggle_class(&self, class: &str) -> bool {
        self.as_element().class_list().toggle(class).unwrap_js()
    }

    #[track_caller]
    fn set_attr(&self, name: &str, value: &str) {
        self.as_element().set_attribute(name, value).unwrap_js();
    }

    fn remove(&self) {
        self.as_element().remove()
    }

    #[track_caller]
    fn children(&self) -> Vec<Dom<Element>> {
        let collection = self.as_element().children();
        let len = collection.length();
        (0..len)
            .map(|i| {
                let child = collection.item(i).expect_throw("invalid index");
                Dom::from(child)
            })
            .collect()
    }

    #[track_caller]
    fn query_selector(&self, selector: &str) -> Vec<Dom<Node>> {
        let node_list = self.as_element().query_selector_all(selector).unwrap_js();
        let len = node_list.length();
        (0..len)
            .map(|i| {
                let node = node_list.item(i).expect_throw("invalid index");
                Dom::from(node)
            })
            .collect()
    }
}

pub trait DomElementBuilderExt<T> {
    fn id(self, id: &str) -> Self;
    fn class(self, class: &str) -> Self;
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: IsElement> DomElementBuilderExt<T> for Dom<T> {
    fn id(self, id: &str) -> Self {
        self.set_id(id);
        self
    }

    #[track_caller]
    fn class(self, class: &str) -> Self {
        self.add_class(class);
        self
    }

    #[track_caller]
    fn attr(self, name: &str, value: &str) -> Self {
        self.set_attr(name, value);
        self
    }
}
