use super::*;

pub trait DomNodeExt<T> {
    fn append(&self, dom: &Dom<impl IsNode>);
    fn set_text(&self, text: &str);
}

impl<T: IsNode> DomNodeExt<T> for Dom<T> {
    #[track_caller]
    fn append(&self, dom: &Dom<impl IsNode>) {
        self.as_node().append_child(dom.as_node()).unwrap_js();
    }

    fn set_text(&self, text: &str) {
        self.as_node().set_text_content(Some(text));
    }
}

pub trait DomNodeBuildExt<T> {
    fn child(self, dom: &Dom<impl IsNode>) -> Self;
    fn text(self, text: &str) -> Self;
}

impl<T: IsNode> DomNodeBuildExt<T> for Dom<T> {
    #[track_caller]
    fn child(self, dom: &Dom<impl IsNode>) -> Self {
        self.append(dom);
        self
    }

    fn text(self, text: &str) -> Self {
        self.set_text(text);
        self
    }
}
