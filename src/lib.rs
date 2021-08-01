use tokio::sync::Notify;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Clone)]
pub struct FutureBool {
    notify_true: Arc<Notify>,
    notify_false: Arc<Notify>,
    inner: Arc<AtomicBool>
}

impl FutureBool {
    pub fn new(val: bool) -> Self {
        Self {
            notify_true: Arc::new(Notify::new()),
            notify_false: Arc::new(Notify::new()),
            inner: Arc::new(AtomicBool::new(val))
        }
    }
    pub fn set(&self) {
        self.inner.store(true, Ordering::Release);
        self.notify_true.notify_waiters();
    }
    pub fn unset(&self) {
        self.inner.store(false, Ordering::Release);
        self.notify_false.notify_waiters();
    }
    pub async fn wait_change(&self) -> bool {
        let val = self.inner.load(Ordering::Acquire);
        if val {
            self.notify_false.notified().await;
        } else {
            self.notify_true.notified().await;
        }
        !val
    }
    pub async fn wait_true(&self) {
        let val = self.inner.load(Ordering::Acquire);
        if !val {
            self.notify_true.notified().await;
        }
    }
    pub async fn wait_false(&self) {
        let val = self.inner.load(Ordering::Acquire);
        if val {
            self.notify_false.notified().await;
        }
    }
}
