use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub request_count: Arc<RwLock<u64>>,
    //RWLock allows multiple readers or one writer at a time
    //Arc allows shared ownership across threads or requests
}

impl AppState {
    pub fn new() -> Self {
        Self {
            request_count: Arc::new(RwLock::new(0)),
        }
    }

    pub fn increment_requests(&self) {
        let mut count = self.request_count.write().unwrap();
        *count += 1;
    }

    pub fn get_request(&self) -> u64 {
        *self.request_count.read().unwrap()
    }
}
