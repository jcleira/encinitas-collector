use std::error::Error;

use r2d2_redis::{r2d2, RedisConnectionManager};

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

        conn.set("foo", "bar").unwrap();

        let n: i64 = conn.incr("counter", 1).unwrap();

        let reply = redis::cmd("PING")
            .query::<String>(conn.deref_mut())
            .unwrap();

        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(&json_message)
            .query(conn.deref_mut())?;

        Ok(())
    }
}
