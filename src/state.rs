use crate::Emitter;
use futures_lite::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct State<T> {
    inner: Rc<SharedInner<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateChange<T> {
    pub value: T,
    pub old_value: T,
}

struct SharedInner<T> {
    value: Cell<T>,
    emitter: Emitter<StateChange<T>>,
}

impl<T: Copy + Eq + Default> Default for State<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Copy + Eq> State<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self::new_with_capacity(value, 1)
    }

    #[inline]
    pub fn new_with_capacity(value: T, capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must not be 0");

        let value = Cell::new(value);
        let emitter = Emitter::new(capacity);
        let inner = Rc::new(SharedInner { value, emitter });

        Self { inner }
    }

    #[inline]
    pub fn get(&self) -> T {
        self.inner.value.get()
    }

    #[inline]
    pub fn set(&self, value: T) {
        let old_value = self.inner.value.replace(value);
        if old_value != value {
            self.inner.emitter.emit(StateChange { value, old_value });
        }
    }

    #[inline]
    pub fn watch(&self) -> impl Stream<Item = StateChange<T>> + use<T> {
        self.inner.emitter.receive()
    }
}

impl State<bool> {
    #[inline]
    pub fn toggle(&self) -> bool {
        self.set(!self.get());
        self.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    async fn i32() {
        let state = State::<i32>::new_with_capacity(0, usize::MAX);
        assert_eq!(state.get(), 0);

        let mut change = state.watch();

        state.set(1);
        assert_eq!(state.get(), 1);
        assert_eq!(
            change.next().await,
            Some(StateChange {
                value: 1,
                old_value: 0
            })
        );
    }

    #[wasm_bindgen_test]
    #[allow(dead_code)]
    async fn bool() {
        let state = State::<bool>::new_with_capacity(false, usize::MAX);
        assert!(!state.get());

        let mut change = state.watch();

        state.set(true);
        assert!(state.get());
        assert_eq!(
            change.next().await,
            Some(StateChange {
                value: true,
                old_value: false
            })
        );

        state.toggle();
        assert!(!state.get());
        assert_eq!(
            change.next().await,
            Some(StateChange {
                value: false,
                old_value: true
            })
        );
    }
}
