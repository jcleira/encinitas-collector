use serde_json;

pub struct EventData {
    pub id: String,
    pub client_id: String,
    pub handled: serde_json::Value,
    pub replaces_client_id: Option<String>,
    pub resulting_client_id: String,
}

pub struct RequestData {
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

pub struct ResponseData {
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

pub struct Event {
    pub event: EventData,
    pub request: Option<RequestData>,
    pub response: Option<ResponseData>,
}
