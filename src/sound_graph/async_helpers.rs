use futures::Future;
use futures::FutureExt;
use std::cell::Cell;
use std::panic;
use std::rc::Rc;
use std::thread;
use wasm_bindgen_futures;

pub struct Task<T>(Rc<Cell<Option<thread::Result<T>>>>);

impl<T: 'static> Task<T> {
    pub fn spawn<F: 'static + Future<Output = T>>(future: F) -> Self {
        let sender = Rc::new(Cell::new(None));
        let receiver = sender.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let future = panic::AssertUnwindSafe(future).catch_unwind();
            sender.set(Some(future.await));
        });
        Self(receiver)
    }
    pub fn take_output(&self) -> Option<thread::Result<T>> {
        self.0.take()
    }
}
