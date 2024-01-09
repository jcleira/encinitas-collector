use crate::infrastructure::repositories::redis::RedisRepository;

pub struct EventsConsumer {
    redis_repository: RedisRepository,
}

impl EventsConsumer {
    pub fn new(redis_repository: RedisRepository) -> Self {
        Self { redis_repository }
    }

    pub async fn consume(&self) {
        match self.redis_repository.subscribe("events") {
            Ok(mut rx) => {
                while let Some(message) = rx.recv().await {
                    println!("Received message: {}", message);
                }
            }
            Err(e) => {
                eprintln!("Error subscribing to channel: {}", e);
            }
        }
    }
}
