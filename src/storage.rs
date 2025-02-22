use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Storage(pub(crate) web_sys::Storage);

impl Storage {
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.0.get_item(key).unwrap_throw()
    }

    #[inline]
    pub fn set(&self, key: &str, value: &str) {
        self.0.set_item(key, value).unwrap_throw()
    }

    #[inline]
    pub fn remove(&self, key: &str) {
        self.0.remove_item(key).unwrap_throw()
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = String> + use<> {
        let storage = self.0.clone();
        let len = storage.length().unwrap_throw();
        (0..len).filter_map(move |idx| storage.key(idx).unwrap_throw())
    }
}
