use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub influxdb_url: String,
    pub influxdb_db: String,
    pub influxdb_token: String,
    pub database_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(Self {
            influxdb_url: env::var("INFLUXDB_URL")
                .unwrap_or_else(|_| "http://localhost:8086".to_string()),
            influxdb_db: env::var("INFLUXDB_DB").unwrap_or_else(|_| "encinitas".to_string()),
            influxdb_token: env::var("INFLUXDB_TOKEN").unwrap_or_else(|_| "".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string()),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost".to_string()),
        })
    }
}
