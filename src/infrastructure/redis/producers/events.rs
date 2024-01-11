use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsProducer {
    redis_repository: RedisRepository,
}

impl EventsProducer {
    pub fn new(redis_repository: RedisRepository) -> Self {
        Self { redis_repository }
    }

    pub fn produce(&self, event_type: &str, event: &Event) -> Result<(), Box<dyn Error>> {
        let redis_event = RedisEvent::from_aggregate(event, event_type);

        self.redis_repository.publish("events", &redis_event)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RedisEvent {
    pub id: String,
    pub client_id: String,
    pub event_type: String,
    pub replaces_client_id: Option<String>,
    pub resulting_client_id: String,
}

impl RedisEvent {
    pub fn to_aggregate(&self) -> Event {
        Event {
            id: self.id.to_string(),
            client_id: self.client_id.to_string(),
            handled: serde_json::Value::Null,
            replaces_client_id: self.replaces_client_id.to_owned(),
            resulting_client_id: self.resulting_client_id.to_owned(),
            request: None,
            response: None,
        }
    }

    pub fn from_aggregate(event: &Event, event_type: &str) -> Self {
        Self {
            event_type: event_type.to_string(),
            id: event.id.to_string(),
            client_id: event.client_id.to_string(),
            replaces_client_id: event.replaces_client_id.to_owned(),
            resulting_client_id: event.resulting_client_id.to_owned(),
        }
    }
}
