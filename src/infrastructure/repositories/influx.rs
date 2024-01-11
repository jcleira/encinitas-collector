use crate::domain::aggregates::event::Event;
use chrono::Utc;
use influxdb::{Client, Timestamp, WriteQuery};
use std::error::Error;

pub struct InfluxRepository {
    client: Client,
}

impl InfluxRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        let timestamp = Timestamp::Milliseconds(Utc::now().timestamp_millis().try_into()?);

        let mut query = WriteQuery::new(timestamp, "event");
        query = query
            .add_field("id", event.id.to_string())
            .add_field("client_id", event.client_id.clone())
            .add_field("replaces_client_id", event.replaces_client_id.clone())
            .add_field("resulting_client_id", event.resulting_client_id.clone());

        if let Some(request) = &event.request {
            query = query
                .add_field("request_body", request.body.clone().unwrap_or_default())
                .add_field("request_body_used", request.body_used.clone())
                .add_field("request_cache", request.cache.clone())
                .add_field("request_credentials", request.credentials.clone())
                .add_field("request_destination", request.destination.clone())
                .add_field("request_integrity", request.integrity.clone())
                .add_field("request_method", request.method.clone())
                .add_field("request_mode", request.mode.clone())
                .add_field("request_redirect", request.redirect.clone())
                .add_field("request_referrer", request.referrer.clone())
                .add_field("request_referrer_policy", request.referrer_policy.clone())
                .add_field("request_url", request.url.clone())
        }

        if let Some(response) = &event.response {
            query = query
                .add_field("response_body", response.body.clone().unwrap_or_default())
                .add_field("response_body_used", response.body_used)
                .add_field("response_ok", response.ok)
                .add_field("response_redirected", response.redirected)
                .add_field("response_status", response.status)
                .add_field("response_status_text", response.status_text.clone())
                .add_field("response_response_type", response.response_type.clone())
                .add_field("response_url", response.url.clone());
        }

        let result = self.client.query(&query).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
