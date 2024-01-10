use std::error::Error;
use std::ops::DerefMut;

use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use serde::Serialize;
use serde_json;
use tokio::sync::mpsc;

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

    pub fn subscribe(&self, channel: &str) -> mpsc::Receiver<String> {
        let channel = channel.to_string();
        let pool = self.pool.clone();
        let (tx, rx) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => return,
            };

            let mut pubsub = conn.as_pubsub();
            if let Err(_) = pubsub.subscribe(channel) {
                return;
            }

            loop {
                match pubsub.get_message() {
                    Ok(message) => {
                        let payload: String = message.get_payload().unwrap_or_default();
                        if tx.send(payload).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        rx
    }
}
