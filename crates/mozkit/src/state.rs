use crate::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct State<T: Copy + Eq> {
    inner: Rc<Inner<T>>,
}

#[derive(Clone)]
pub struct ReadOnlyState<T: Copy + Eq> {
    inner: Rc<Inner<T>>,
}

struct Inner<T: Copy + Eq> {
    value: Cell<T>,
    update_event: Event<(T, T)>,
}

impl<T: Copy + Eq + Default> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Copy + Eq> State<T> {
    pub fn new(value: T) -> Self {
        Self::new_with_capacity(value, 1)
    }

    pub fn new_with_capacity(value: T, capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must not be 0");

        let value = Cell::new(value);
        let update_event = Event::new(capacity);
        let inner = Rc::new(Inner {
            value,
            update_event,
        });

        Self { inner }
    }

    pub fn downgrade(self) -> ReadOnlyState<T> {
        ReadOnlyState { inner: self.inner }
    }

    pub fn get(&self) -> T {
        self.inner.value.get()
    }

    #[track_caller]
    pub fn set(&self, value: T) {
        let old = self.inner.value.replace(value);
        if old != value {
            self.inner.update_event.emit((value, old));
        }
    }

    pub fn watch(&self) -> impl Stream<Item = (T, T)> {
        self.inner.update_event.listen()
    }

    pub fn watch_value(&self) -> impl Stream<Item = T> {
        self.watch().map(|(value, _)| value)
    }
}

impl<T: Copy + Eq> ReadOnlyState<T> {
    pub fn get(&self) -> T {
        self.inner.value.get()
    }

    pub fn watch(&self) -> impl Stream<Item = (T, T)> {
        self.inner.update_event.listen()
    }

    pub fn watch_value(&self) -> impl Stream<Item = T> {
        self.watch().map(|(value, _)| value)
    }
}

impl State<bool> {
    #[track_caller]
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
    async fn i32() {
        let state = State::<i32>::new_with_capacity(0, usize::MAX);
        assert_eq!(state.get(), 0);

        let mut change_stream = state.watch();
        let mut value_stream = state.watch_value();

        state.set(1);
        assert_eq!(state.get(), 1);
        assert_eq!(change_stream.next().await, Some((1, 0)));
        assert_eq!(value_stream.next().await, Some(1));
    }

    #[wasm_bindgen_test]
    #[allow(clippy::bool_assert_comparison)]
    async fn bool() {
        let state = State::<bool>::new_with_capacity(false, usize::MAX);
        assert_eq!(state.get(), false);

        let mut change_stream = state.watch();
        let mut value_stream = state.watch_value();

        state.set(true);
        assert_eq!(state.get(), true);
        assert_eq!(change_stream.next().await, Some((true, false)));
        assert_eq!(value_stream.next().await, Some(true));

        state.toggle();
        assert_eq!(state.get(), false);
        assert_eq!(change_stream.next().await, Some((false, true)));
        assert_eq!(value_stream.next().await, Some(false));
    }
}
