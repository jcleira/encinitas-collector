use diesel::insertable::Insertable;
use diesel::{allow_tables_to_appear_in_same_query, joinable, table};
use uuid::Uuid;

use crate::domain::aggregates::event::{EventData, RequestData, ResponseData};

#[derive(Insertable)]
#[table_name = "events"]
pub struct DBEvent {
    pub id: Uuid,
    pub browser_id: String,
    pub client_id: String,
    pub handled: serde_json::Value,
    pub replaces_client_id: Option<String>,
    pub resulting_client_id: String,
}

pub fn db_event_from_aggregate(event: &EventData) -> DBEvent {
    DBEvent {
        id: Uuid::new_v4(),
        browser_id: event.id.clone(),
        client_id: event.client_id.clone(),
        handled: event.handled.clone(),
        replaces_client_id: event.replaces_client_id.clone(),
        resulting_client_id: event.resulting_client_id.clone(),
    }
}

#[derive(Insertable)]
#[table_name = "requests"]
pub struct DBRequest {
    pub id: Uuid,
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

pub fn db_request_from_aggregate(request: &RequestData) -> DBRequest {
    DBRequest {
        id: Uuid::new_v4(),
        body: request.body.clone(),
        body_used: request.body_used,
        cache: request.cache.clone(),
        credentials: request.credentials.clone(),
        destination: request.destination.clone(),
        headers: request.headers.clone(),
        integrity: request.integrity.clone(),
        method: request.method.clone(),
        mode: request.mode.clone(),
        redirect: request.redirect.clone(),
        referrer: request.referrer.clone(),
        referrer_policy: request.referrer_policy.clone(),
        url: request.url.clone(),
        signal: request.signal.clone(),
    }
}

#[derive(Insertable)]
#[table_name = "responses"]
pub struct DBResponse {
    pub id: Uuid,
    pub body: Option<String>,
    pub body_used: bool,
    pub headers: serde_json::Value,
    pub ok: bool,
    pub redirected: bool,
    pub status: i32,
    pub status_text: String,
    pub response_type: String,
    pub url: String,
}

pub fn db_response_from_aggregate(response: &ResponseData) -> DBResponse {
    DBResponse {
        id: Uuid::new_v4(),
        body: response.body.clone(),
        body_used: response.body_used,
        headers: response.headers.clone(),
        ok: response.ok,
        redirected: response.redirected,
        status: response.status as i32,
        status_text: response.status_text.clone(),
        response_type: response.response_type.clone(),
        url: response.url.clone(),
    }
}

table! {
    events (id) {
        id -> Uuid,
        browser_id -> Varchar,
        client_id -> Varchar,
        handled -> Jsonb,
        replaces_client_id -> Varchar,
        resulting_client_id -> Varchar,
    }
}

table! {
    requests (event_id) {
        id -> Uuid,
        event_id -> Uuid,
        body -> Nullable<Text>,
        body_used -> Bool,
        cache -> Varchar,
        credentials -> Varchar,
        destination -> Varchar,
        headers -> Jsonb,
        integrity -> Varchar,
        method -> Varchar,
        mode -> Varchar,
        redirect -> Varchar,
        referrer -> Varchar,
        referrer_policy -> Varchar,
        url -> Varchar,
        signal -> Jsonb,
    }
}

table! {
    responses (event_id) {
        id -> Uuid,
        event_id -> Uuid,
        body -> Nullable<Text>,
        body_used -> Bool,
        headers -> Jsonb,
        ok -> Bool,
        redirected -> Bool,
        status -> Int4,
        status_text -> Varchar,
        response_type -> Varchar,
        url -> Varchar,
    }
}

joinable!(requests -> events (event_id));
joinable!(responses -> events (event_id));

allow_tables_to_appear_in_same_query!(events, requests, responses,);
