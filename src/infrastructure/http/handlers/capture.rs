use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::domain::aggregates::event::{Event, Request, Response};
use crate::domain::services::events_creator::EventsCreator;

pub struct CaptureEndpoint {
    events_creator: EventsCreator,
}

impl CaptureEndpoint {
    pub fn new(events_creator: EventsCreator) -> Self {
        Self { events_creator }
    }

    pub async fn endpoint(&self, http_event: web::Json<HTTPEvent>) -> impl Responder {
        println!("http event: {:?}", http_event);
        let event = http_event.to_aggregate();
        let _ = self.events_creator.create(&event).await.map_err(|e| {
            println!("failed to create event: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        });

        HttpResponse::Ok().body("OK")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTTPEvent {
    id: String,
    #[serde(rename = "clientId")]
    client_id: String,
    handled: serde_json::Value,
    #[serde(rename = "replacesClientId")]
    replaces_client_id: Option<String>,
    #[serde(rename = "resultingClientId")]
    resulting_client_id: String,
    request: Option<HTTPRequestData>,
    response: Option<HTTPResponseData>,
}

impl HTTPEvent {
    pub fn to_aggregate(&self) -> Event {
        Event {
            id: Uuid::new_v4(),
            browser_id: self.id.clone(),
            client_id: self.client_id.clone(),
            handled: self.handled.clone(),
            replaces_client_id: self.replaces_client_id.clone(),
            resulting_client_id: self.resulting_client_id.clone(),
            request: self.request.as_ref().map(|req| req.to_aggregate()), // Transforms to Option<Request>
            response: self.response.as_ref().map(|res| res.to_aggregate()), // Handling Option
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HTTPRequestData {
    body: Option<String>,
    #[serde(rename = "bodyUsed")]
    body_used: bool,
    cache: String,
    credentials: String,
    destination: String,
    headers: serde_json::Value,
    integrity: String,
    method: String,
    mode: String,
    redirect: String,
    referrer: String,
    #[serde(rename = "referrerPolicy")]
    referrer_policy: String,
    signal: serde_json::Value,
    url: String,
}

impl HTTPRequestData {
    pub fn to_aggregate(&self) -> Request {
        Request {
            body: self.body.clone(),
            body_used: self.body_used.clone(),
            cache: self.cache.clone(),
            credentials: self.credentials.clone(),
            destination: self.destination.clone(),
            headers: self.headers.clone(),
            integrity: self.integrity.clone(),
            method: self.method.clone(),
            mode: self.mode.clone(),
            redirect: self.redirect.clone(),
            referrer: self.referrer.clone(),
            referrer_policy: self.referrer_policy.clone(),
            signal: self.signal.clone(),
            url: self.url.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HTTPResponseData {
    body: Option<String>,
    #[serde(rename = "bodyUsed")]
    body_used: bool,
    headers: serde_json::Value,
    ok: bool,
    redirected: bool,
    status: u16,
    #[serde(rename = "statusText")]
    status_text: String,
    #[serde(rename = "type")]
    response_type: String,
    url: String,
}

impl HTTPResponseData {
    pub fn to_aggregate(&self) -> Response {
        Response {
            body: self.body.clone(),
            body_used: self.body_used.clone(),
            headers: self.headers.clone(),
            ok: self.ok,
            redirected: self.redirected,
            status: self.status.clone(),
            status_text: self.status_text.clone(),
            response_type: self.response_type.clone(),
            url: self.url.clone(),
        }
    }
}
