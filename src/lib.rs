use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Notify;

/// A bool one can `await` the changes.
#[derive(Clone)]
pub struct FutureBool {
    notify_true: Arc<Notify>,
    notify_false: Arc<Notify>,
    inner: Arc<AtomicBool>,
}

impl FutureBool {
    /// Creates a new `FutureBool` with some initial value.
    pub fn new(val: bool) -> Self {
        Self {
            notify_true: Arc::new(Notify::new()),
            notify_false: Arc::new(Notify::new()),
            inner: Arc::new(AtomicBool::new(val)),
        }
    }

    /// Sets the `bool` value to `true`.
    pub fn set(&self) {
        self.inner.store(true, Ordering::Release);
        self.notify_true.notify_waiters();
    }

    /// Sets the `bool` value to `false`.
    pub fn unset(&self) {
        self.inner.store(false, Ordering::Release);
        self.notify_false.notify_waiters();
    }

    /// Returns the new value when it has changed.
    pub async fn wait_change(&self) -> bool {
        let val = self.inner.load(Ordering::Acquire);
        if val {
            self.notify_false.notified().await;
        } else {
            self.notify_true.notified().await;
        }
        !val
    }

    /// If the value is `true`, returns immidiately, otherwise waits until it's `true`.
    pub async fn wait_true(&self) {
        let val = self.inner.load(Ordering::Acquire);
        if !val {
            self.notify_true.notified().await;
        }
    }

    /// If the value is `false`, returns immidiately, otherwise waits until it's `false`.
    pub async fn wait_false(&self) {
        let val = self.inner.load(Ordering::Acquire);
        if val {
            self.notify_false.notified().await;
        }
    }
}
