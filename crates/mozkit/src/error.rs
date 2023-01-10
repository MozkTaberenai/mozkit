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

// #[derive(Debug)]
// pub struct JsError(js_sys::Error);

// impl std::fmt::Display for JsError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "JsError: {}", self.0.to_string())
//     }
// }

// impl std::error::Error for JsError {}

// impl From<js_sys::Error> for JsError {
//     fn from(js_error: js_sys::Error) -> Self {
//         Self(js_error)
//     }
// }

// #[derive(Debug)]
// pub struct JsValueError(wasm_bindgen::JsValue);

// impl std::fmt::Display for JsValueError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "JsValueError: {}",
//             js_sys::JsString::from(self.0.clone())
//         )
//     }
// }

// impl std::error::Error for JsValueError {}

// impl From<wasm_bindgen::JsValue> for JsValueError {
//     fn from(js_value: wasm_bindgen::JsValue) -> Self {
//         Self(js_value)
//     }
// }

// from https://github.com/Pauan/rust-dominator/blob/master/src/utils.rs
pub trait UnwrapJsExt<T> {
    fn unwrap_js(self) -> T;
}

impl<T> UnwrapJsExt<T> for Result<T, JsValue> {
    #[track_caller]
    fn unwrap_js(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => match e.dyn_ref::<js_sys::Error>() {
                Some(e) => panic!("{}", e.message()),
                None => panic!("{:?}", e),
            },
        }
    }
}
