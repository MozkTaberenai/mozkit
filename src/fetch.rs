use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub enum Error {
    Client(u16, String),
    Server(u16, String),
    Js(JsValue),
}

impl From<JsValue> for Error {
    #[inline]
    fn from(js_value: JsValue) -> Self {
        Self::Js(js_value)
    }
}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Client(status, status_text) => write!(f, "ClientError: {status} {status_text}"),
            Error::Server(status, status_text) => write!(f, "ServerError: {status} {status_text}"),
            Error::Js(inner) => write!(f, "{inner:?}"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Response {
    inner: web_sys::Response,
}

impl TryFrom<web_sys::Response> for Response {
    type Error = Error;

    #[inline]
    fn try_from(inner: web_sys::Response) -> Result<Self, Self::Error> {
        if inner.ok() {
            Ok(Self { inner })
        } else {
            let status = inner.status();
            match status {
                400..=499 => Err(Error::Client(status, inner.status_text())),
                500..=599 => Err(Error::Server(status, inner.status_text())),
                _ => unreachable!(),
            }
        }
    }
}

impl Response {
    #[inline]
    pub fn ok(&self) -> bool {
        self.inner.ok()
    }

    #[inline]
    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    #[inline]
    pub fn status_text(&self) -> String {
        self.inner.status_text()
    }

    #[inline]
    pub async fn body(&self) -> Result<Vec<u8>, Error> {
        let array_buffer = JsFuture::from(self.inner.array_buffer()?).await?;
        Ok(js_sys::Uint8Array::new(&array_buffer).to_vec())
    }
}

#[inline]
pub async fn get(url: &str) -> Result<Response, Error> {
    let promise = crate::window().fetch_with_str_and_init(url, &web_sys::RequestInit::new());
    let res = JsFuture::from(promise)
        .await?
        .dyn_into::<web_sys::Response>()?;
    Response::try_from(res)
}

#[inline]
pub async fn post(url: &str, body: Option<&[u8]>) -> Result<Response, Error> {
    use js_sys::Uint8Array;
    let body = match body {
        Some(bytes) => {
            let u8a = Uint8Array::new_with_length(bytes.len() as u32);
            u8a.copy_from(bytes);
            u8a
        }
        None => Uint8Array::new_with_length(0),
    };

    let mut request_init = web_sys::RequestInit::new();
    request_init.method("POST").body(Some(&body));

    let promise = crate::window().fetch_with_str_and_init(url, &request_init);

    let res = JsFuture::from(promise)
        .await?
        .dyn_into::<web_sys::Response>()?;

    Response::try_from(res)
}
