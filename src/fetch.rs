use super::{window, FromJsError, JsCast, JsFuture, JsValue};
use js_sys::Uint8Array;

#[derive(Debug)]
pub enum Error {
    Client(u16, String),
    Server(u16, String),
    FromJs(FromJsError),
}

impl From<JsValue> for Error {
    fn from(js_value: JsValue) -> Self {
        Self::FromJs(FromJsError::from(js_value))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Client(status, status_text) => write!(f, "ClientError: {status} {status_text}"),
            Error::Server(status, status_text) => write!(f, "ServerError: {status} {status_text}"),
            Error::FromJs(inner) => write!(f, "{inner}"),
        }
    }
}

impl std::error::Error for Error {}

pub struct Response {
    inner: web_sys::Response,
}

impl TryFrom<web_sys::Response> for Response {
    type Error = Error;

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
    pub fn ok(&self) -> bool {
        self.inner.ok()
    }

    pub fn status(&self) -> u16 {
        self.inner.status()
    }

    pub fn status_text(&self) -> String {
        self.inner.status_text()
    }

    pub async fn body(&self) -> Result<Vec<u8>, Error> {
        let array_buffer = JsFuture::from(self.inner.array_buffer()?).await?;
        Ok(Uint8Array::new(&array_buffer).to_vec())
    }
}

pub async fn get(url: &str) -> Result<Response, Error> {
    let res = JsFuture::from(window().fetch_with_str(url))
        .await?
        .dyn_into::<web_sys::Response>()?;
    Response::try_from(res)
}

pub async fn post(url: &str, body: Option<&[u8]>) -> Result<Response, Error> {
    let mut request_init = web_sys::RequestInit::new();
    request_init.method("POST");

    if let Some(slice) = body {
        let u8a = Uint8Array::new_with_length(slice.len() as u32);
        u8a.copy_from(slice);
        let ab = u8a.buffer();
        request_init.body(Some(ab.as_ref()));
    }

    let res = JsFuture::from(window().fetch_with_str_and_init(url, &request_init))
        .await?
        .dyn_into::<web_sys::Response>()?;

    Response::try_from(res)
}
