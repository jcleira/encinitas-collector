use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::domain::aggregates::event::{Event, EventData, RequestData, ResponseData};
use crate::domain::services::events_creator::EventsCreator;

pub struct CaptureEndpoint {
    events_creator: EventsCreator,
}

impl CaptureEndpoint {
    pub fn new(events_creator: EventsCreator) -> Self {
        Self { events_creator }
    }

    pub async fn endpoint(&self, http_event: web::Json<HTTPEvent>) -> impl Responder {
        let event = http_event.to_aggregate();
        let _ = self.events_creator.create(&event).await.map_err(|e| {
            println!("failed to create event: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        });

        HttpResponse::Ok().body("OK")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HTTPEventData {
    #[serde(rename = "type")]
    event_type: String,
    request: String,
    client_id: String,
    preload_response: Option<String>,
    resulting_client_id: String,
    replaces_client_id: String,
}

impl HTTPEventData {
    pub fn to_aggregate(&self) -> EventData {
        EventData {
            event_type: self.event_type.clone(),
            request: self.request.clone(),
            client_id: self.client_id.clone(),
            preload_response: self.preload_response.clone(),
            resulting_client_id: self.resulting_client_id.clone(),
            replaces_client_id: self.replaces_client_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HTTPRequestData {
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

impl HTTPRequestData {
    pub fn to_aggregate(&self) -> RequestData {
        RequestData {
            url: self.url.clone(),
            method: self.method.clone(),
            body: self.body.clone(),
            referrer: self.referrer.clone(),
            referrer_policy: self.referrer_policy.clone(),
            mode: self.mode.clone(),
            credentials: self.credentials.clone(),
            cache: self.cache.clone(),
            redirect: self.redirect.clone(),
            integrity: self.integrity.clone(),
            keepalive: self.keepalive,
            signal: self.signal.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HTTPResponseData {
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

impl HTTPResponseData {
    pub fn to_aggregate(&self) -> ResponseData {
        ResponseData {
            url: self.url.clone(),
            response_type: self.response_type.clone(),
            status: self.status,
            ok: self.ok,
            status_text: self.status_text.clone(),
            body: self.body.clone(),
            redirected: self.redirected,
            body_used: self.body_used,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPEvent {
    event: HTTPEventData,
    request: HTTPRequestData,
    response: HTTPResponseData,
}

impl HTTPEvent {
    pub fn to_aggregate(&self) -> Event {
        Event {
            event: self.event.to_aggregate(),
            request: self.request.to_aggregate(),
            response: self.response.to_aggregate(),
        }
    }
}
