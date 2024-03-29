use core::sync::atomic::Ordering;
use std::sync::{atomic::AtomicUsize, Arc};

#[derive(Clone, Debug)]
pub struct AppState {
    pub counter: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

pub trait CounterResourceAccess {
    fn get_counter(&self) -> usize;
    fn increment_counter(&self) -> usize;
}

impl CounterResourceAccess for AppState {
    fn get_counter(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    fn increment_counter(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
