use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug)]
pub enum FromJsError {
    JsError(js_sys::Error),
    JsValue(JsValue),
}

impl std::fmt::Display for FromJsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromJsError::JsError(js_error) => write!(f, "JsError: {}", js_error.to_string()),
            FromJsError::JsValue(js_value) => {
                write!(f, "JsValue: {}", js_sys::JsString::from(js_value.clone()))
            }
        }
    }
}

impl std::error::Error for FromJsError {}

impl From<JsValue> for FromJsError {
    fn from(js_value: JsValue) -> Self {
        match js_value.dyn_into::<js_sys::Error>() {
            Ok(js_error) => Self::JsError(js_error),
            Err(js_value) => Self::JsValue(js_value),
        }
    }
}

// from https://github.com/Pauan/rust-dominator/blob/master/src/utils.rs
pub trait UnwrapJsExt<T> {
    fn unwrap_js(self) -> T;
}

impl<T> UnwrapJsExt<T> for Result<T, JsValue> {
    #[track_caller]
    fn unwrap_js(self) -> T {
        match self {
            Ok(value) => value,
            Err(js_value) => match js_value.dyn_ref::<js_sys::Error>() {
                Some(js_error) => panic!("{}", js_error.message()),
                None => panic!("{js_value:?}"),
            },
        }
    }
}
