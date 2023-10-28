use crate::*;

#[derive(Debug, Clone)]
pub struct Storage(web_sys::Storage);

impl From<web_sys::Storage> for Storage {
    fn from(inner: web_sys::Storage) -> Self {
        Self(inner)
    }
}

impl Storage {
    pub fn get(&self, key: &str) -> Option<String> {
        self.0.get_item(key).unwrap_js()
    }

    pub fn set(&self, key: &str, value: &str) {
        self.0.set_item(key, value).unwrap_js()
    }

    pub fn remove(&self, key: &str) {
        self.0.remove_item(key).unwrap_js()
    }

    pub fn keys(&self) -> impl Iterator<Item = String> {
        let storage = self.0.clone();
        let len = storage.length().unwrap_js();
        (0..len).filter_map(move |idx| storage.key(idx).unwrap_js())
    }
}
