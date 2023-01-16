use crate::*;

mod init {
    use super::*;

    pub fn window() -> web_sys::Window {
        web_sys::window().expect_throw("should never fail: DOM runtime maybe broken.")
    }

    pub fn document() -> web_sys::Document {
        WINDOW.with(|window| {
            window
                .document()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn history() -> web_sys::History {
        WINDOW.with(|window| {
            window
                .history()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn local_storage() -> web_sys::Storage {
        WINDOW.with(|window| {
            window
                .local_storage()
                .unwrap_js()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn session_storage() -> web_sys::Storage {
        WINDOW.with(|window| {
            window
                .session_storage()
                .unwrap_js()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn document_element() -> web_sys::Element {
        DOCUMENT.with(|document| {
            document
                .document_element()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn head() -> web_sys::HtmlHeadElement {
        DOCUMENT.with(|document| {
            document
                .head()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }

    pub fn body() -> web_sys::HtmlElement {
        DOCUMENT.with(|document| {
            document
                .body()
                .expect_throw("should never fail: DOM runtime maybe broken.")
        })
    }
}

thread_local! {
    static WINDOW: Dom<web_sys::Window> = Dom::from(init::window());
    static DOCUMENT: Dom<web_sys::Document> = Dom::from(init::document());
    static HISTORY: web_sys::History = init::history();
    static LOCAL_STORAGE: Storage = Storage::from(init::local_storage());
    static SESSION_STORAGE: Storage = Storage::from(init::session_storage());
    static DOCUMENT_ELEMENT: Dom<Element> = Dom::from(init::document_element());
    static HEAD: Dom<web_sys::HtmlHeadElement> = Dom::from(init::head());
    static BODY: Dom<HtmlElement> = Dom::from(init::body());
}

pub fn window() -> Dom<web_sys::Window> {
    WINDOW.with(|v| v.clone())
}

pub fn document() -> Dom<web_sys::Document> {
    DOCUMENT.with(|v| v.clone())
}

pub fn history() -> web_sys::History {
    HISTORY.with(|v| v.clone())
}

pub fn local_storage() -> Storage {
    LOCAL_STORAGE.with(|v| v.clone())
}

pub fn session_storage() -> Storage {
    SESSION_STORAGE.with(|v| v.clone())
}

pub fn document_element() -> Dom<web_sys::Element> {
    DOCUMENT_ELEMENT.with(|v| v.clone())
}

pub fn head() -> Dom<web_sys::HtmlHeadElement> {
    HEAD.with(|v| v.clone())
}

pub fn body() -> Dom<web_sys::HtmlElement> {
    BODY.with(|v| v.clone())
}
