use crate::*;
use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

pub struct Interval {
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
}

impl Interval {
    pub fn new(millisecs: i32) -> Self {
        let state = Rc::new(Cell::new(State::Default));
        let closure = Closure::new({
            let state = state.clone();
            move || {
                let State::Waiting(waker) = state.replace(State::Waked) else { return };
                waker.wake();
            }
        });

        let id = window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
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

impl Drop for Interval {
    fn drop(&mut self) {
        window().clear_interval_with_handle(self.id);
    }
}

impl Stream for Interval {
    type Item = ();

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
