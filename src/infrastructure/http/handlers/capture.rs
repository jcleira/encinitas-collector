use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::domain::aggregates::metrics::AppMetrics;

#[derive(Debug, Serialize, Deserialize)]
struct FetchEvent {
    #[serde(rename = "type")]
    event_type: String,
    request: String,
    client_id: String,
    preload_response: Option<String>,
    resulting_client_id: String,
    replaces_client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestData {
    url: String,
    method: String,
    headers: serde_json::Value,
    body: Option<String>,
    referrer: String,
    referrer_policy: String,
    mode: String,
    credentials: String,
    cache: String,
    redirect: String,
    integrity: String,
    keepalive: bool,
    signal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    url: String,
    #[serde(rename = "type")]
    response_type: String,
    status: u16,
    ok: bool,
    status_text: String,
    headers: serde_json::Value,
    body: Option<String>,
    redirected: bool,
    body_used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebData {
    fetch_event: FetchEvent,
    request: RequestData,
    response: ResponseData,
}

pub async fn capture_endpoint(
    _data: web::Json<WebData>,
    app_metrics: web::Data<AppMetrics>,
) -> impl Responder {
    let metrics = app_metrics.metrics.lock().unwrap();
    metrics.request_count.inc();
    HttpResponse::Ok().body("Data received")
}
