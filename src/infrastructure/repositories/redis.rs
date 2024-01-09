use std::error::Error;
use std::ops::DerefMut;

use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use serde::Serialize;
use serde_json;

pub struct RedisRepository {
    pool: r2d2::Pool<RedisConnectionManager>,
}

impl RedisRepository {
    pub fn new(redis_url: &str) -> Self {
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        RedisRepository {
            pool: r2d2::Pool::builder().build(manager).unwrap(),
        }
    }

    pub fn publish<T: Serialize>(&self, channel: &str, message: &T) -> Result<(), Box<dyn Error>> {
        let json_message = serde_json::to_string(message)?;
        let mut conn = self.pool.get()?;

        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(&json_message)
            .query(conn.deref_mut())?;

        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<&redis::PubSub, Box<dyn Error>> {
        let mut conn = self.pool.get()?;

        let mut pubsub: redis::PubSub = conn.as_pubsub();
        pubsub.subscribe(channel)?;

        return Ok(&pubsub);
    }
}
