use crate::*;
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

pub struct Timeout {
    id: i32,
    state: Rc<Cell<State>>,
    _closure: Closure<dyn FnMut()>,
}

#[derive(Default)]
enum State {
    #[default]
    Default,
    Waiting(Waker),
    Waked,
    Finished,
}

impl Timeout {
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

        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                millisecs,
            )
            .unwrap_js();

        Self {
            id,
            state,
            _closure: closure,
        }
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        window().clear_timeout_with_handle(self.id);
    }
}

impl Future for Timeout {
    type Output = ();

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
