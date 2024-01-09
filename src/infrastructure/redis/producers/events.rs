use std::error::Error;

use serde::Serialize;

use crate::domain::aggregates::event::EventData;
use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsProducer {
    redis_repository: RedisRepository,
}

impl EventsProducer {
    pub fn new(redis_repository: RedisRepository) -> Self {
        Self { redis_repository }
    }

    pub fn produce(&self, event_type: &str, event: &EventData) -> Result<(), Box<dyn Error>> {
        let redis_event = redis_event_from_aggregate(event, event_type);

        self.redis_repository.publish("events", &redis_event)?;

        Ok(())
    }
}

#[derive(Serialize)]
struct RedisEvent {
    pub client_id: String,
    pub event_type: String,
}

impl RedisEvent {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

fn redis_event_from_aggregate(event: &EventData, event_type: &str) -> RedisEvent {
    RedisEvent {
        client_id: event.client_id.to_string(),
        event_type: event_type.to_string(),
    }
}
