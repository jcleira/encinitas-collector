use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsConsumer {
    redis_repository: RedisRepository,
}

impl EventsConsumer {
    pub fn new(redis_repository: RedisRepository) -> Self {
        Self { redis_repository }
    }

    pub async fn consume(&self) -> Result<(), Box<dyn Error>> {
        let pubsub = self.redis_repository.subscribe("events")?;

        loop {
            let msg = pubsub.get_message()?;
        }

        Ok(())
    }
}
