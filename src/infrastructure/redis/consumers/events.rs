use crate::domain::services::events_processor::EventsProcessor;
use crate::infrastructure::redis::producers::events::RedisEvent;
use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsConsumer {
    redis_repository: RedisRepository,
    events_processor: EventsProcessor,
}

impl EventsConsumer {
    pub fn new(redis_repository: RedisRepository, events_processor: EventsProcessor) -> Self {
        Self {
            redis_repository,
            events_processor,
        }
    }

    pub async fn consume(&self) {
        let mut rx = self.redis_repository.subscribe("events");

        while let Some(event_message) = rx.recv().await {
            let event_json: serde_json::Value = serde_json::from_str(&event_message).unwrap();
            let redis_event: RedisEvent = serde_json::from_value(event_json.clone()).unwrap();
            let event = redis_event.to_aggregate();

            match redis_event.event_type.as_str() {
                "created" => self.events_processor.process(event).unwrap(),
                _ => todo!(),
            }

            println!("Received message: {}", event_message);
        }
    }
}
