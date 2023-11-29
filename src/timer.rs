use futures_lite::prelude::*;
use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = setTimeout)]
    fn set_timeout(callback: &js_sys::Function, delay: i32) -> Result<i32, JsValue>;
    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(id: i32);
    #[wasm_bindgen(catch, js_name = setInterval)]
    fn set_interval(callback: &js_sys::Function, delay: i32) -> Result<i32, JsValue>;
    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(id: i32);
}

#[derive(Default)]
enum State {
    #[default]
    Default,
    Waiting(Waker),
    Waked,
    Finished,
}

pub struct Timeout {
    id: i32,
    state: Rc<Cell<State>>,
    _closure: Closure<dyn FnMut()>,
}

impl Timeout {
    #[inline]
    pub fn new(millisecs: i32) -> Self {
        let state = Rc::new(Cell::new(State::Default));
        let closure = Closure::once({
            let state = state.clone();
            move || {
                let State::Waiting(waker) = state.replace(State::Waked) else {
                    return;
                };
                waker.wake();
            }
        });

        let id = set_timeout(closure.as_ref().unchecked_ref(), millisecs).unwrap_throw();

        Self {
            id,
            state,
            _closure: closure,
        }
    }
}

impl Drop for Timeout {
    #[inline]
    fn drop(&mut self) {
        clear_timeout(self.id);
    }
}

impl Future for Timeout {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state.take() {
            State::Finished => unreachable!(),
            State::Waked => {
                self.state.set(State::Finished);
                Poll::Ready(())
            }
            _ => {
                self.state.set(State::Waiting(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}

pub struct Interval {
    id: i32,
    state: Rc<Cell<State>>,
    _closure: Closure<dyn FnMut()>,
}

impl Interval {
    #[inline]
    pub fn new(millisecs: i32) -> Self {
        let state = Rc::new(Cell::new(State::Default));
        let closure = Closure::new({
            let state = state.clone();
            move || {
                let State::Waiting(waker) = state.replace(State::Waked) else {
                    return;
                };
                waker.wake();
            }
        });

        let id = set_interval(closure.as_ref().unchecked_ref(), millisecs).unwrap_throw();

        Self {
            id,
            state,
            _closure: closure,
        }
    }
}

impl Drop for Interval {
    #[inline]
    fn drop(&mut self) {
        clear_interval(self.id);
    }
}

impl Stream for Interval {
    type Item = ();

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state.take() {
            State::Waked => Poll::Ready(Some(())),
            _ => {
                self.state.set(State::Waiting(cx.waker().clone()));
                Poll::Pending
            }
        }
    }
}
