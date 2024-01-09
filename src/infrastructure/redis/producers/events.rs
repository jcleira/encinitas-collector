use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsProducer {
    redis_repository: RedisRepository,
}

impl EventsProducer {
    pub fn new(redis_repository: RedisRepository) -> Self {
        Self { redis_repository }
    }

    pub async fn produce(&self, event: &str) -> Result<(), Box<dyn Error>> {
        self.redis_repository.publish("events", event)?;

        Ok(())
    }
}
