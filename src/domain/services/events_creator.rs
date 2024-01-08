use std::error::Error;

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::influx::InfluxRepository;
use crate::infrastructure::repositories::redis::RedisRepository;
use crate::infrastructure::repositories::sql::PostgresRepository;

pub struct EventsCreator {
    influx_repository: InfluxRepository,
    postgres_repository: PostgresRepository,
    redis_repository: RedisRepository,
}

impl EventsCreator {
    pub fn new(
        influx_repository: InfluxRepository,
        postgres_repository: PostgresRepository,
        redis_repository: RedisRepository,
    ) -> Self {
        Self {
            influx_repository,
            postgres_repository,
            redis_repository,
        }
    }

    pub async fn create(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        self.influx_repository.create(event).await;

        self.postgres_repository.create_event(event)?;
        if event.request.is_some() {
            self.postgres_repository.create_request(event)?;
        }
        if event.response.is_some() {
            self.postgres_repository.create_response(event)?;
        }

        self.redis_repository
            .publish("events", &event.event.to_json())?;

        Ok(())
    }
}
