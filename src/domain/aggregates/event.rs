use serde_json;
use uuid::Uuid;

// Event is the aggregate root, is the representation of an event coming from
// the browser.
//
// The event is composed of a request or a response, and the event information
// itself. The service worker in the frontend send an event to the backend for
// the request and then waits for the response.
pub struct Event {
    pub id: Uuid,
    pub browser_id: String,
    pub client_id: String,
    pub handled: serde_json::Value,
    pub replaces_client_id: Option<String>,
    pub resulting_client_id: String,
    pub request: Option<Request>,
    pub response: Option<Response>,
}

pub struct Request {
    pub body: Option<String>,
    pub body_used: bool,
    pub cache: String,
    pub credentials: String,
    pub destination: String,
    pub headers: serde_json::Value,
    pub integrity: String,
    pub method: String,
    pub mode: String,
    pub redirect: String,
    pub referrer: String,
    pub referrer_policy: String,
    pub url: String,
    pub signal: serde_json::Value,
}

pub struct Response {
    pub body: Option<String>,
    pub body_used: bool,
    pub headers: serde_json::Value,
    pub ok: bool,
    pub redirected: bool,
    pub status: u16,
    pub status_text: String,
    pub response_type: String,
    pub url: String,
}
