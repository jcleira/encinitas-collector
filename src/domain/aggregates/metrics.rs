use prometheus::{opts, register_int_counter, IntCounter};
use std::sync::Mutex;

pub struct Metrics {
    pub request_count: IntCounter,
}

pub struct AppMetrics {
    pub metrics: Mutex<Metrics>,
}

impl Metrics {
    pub fn new() -> Self {
        let request_count =
            register_int_counter!(opts!("request_count", "requests received")).unwrap();
        Metrics { request_count }
    }
}
