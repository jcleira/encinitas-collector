use actix_web::{web, HttpResponse, Responder};
use prometheus::{Encoder, TextEncoder};

use crate::domain::aggregates::metrics::AppMetrics;

pub async fn metrics_endpoint(_app_metrics: web::Data<AppMetrics>) -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok().body(String::from_utf8(buffer).unwrap())
}
