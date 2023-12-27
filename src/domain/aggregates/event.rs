pub struct EventData {
    pub event_type: String,
    pub request: String,
    pub client_id: String,
    pub preload_response: Option<String>,
    pub resulting_client_id: String,
    pub replaces_client_id: String,
}

pub struct RequestData {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub referrer: String,
    pub referrer_policy: String,
    pub mode: String,
    pub credentials: String,
    pub cache: String,
    pub redirect: String,
    pub integrity: String,
    pub keepalive: bool,
    pub signal: Option<String>,
}

pub struct ResponseData {
    pub url: String,
    pub response_type: String,
    pub status: u16,
    pub ok: bool,
    pub status_text: String,
    pub body: Option<String>,
    pub redirected: bool,
    pub body_used: bool,
}

pub struct Event {
    pub event: EventData,
    pub request: RequestData,
    pub response: ResponseData,
}
