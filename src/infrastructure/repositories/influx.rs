use crate::domain::aggregates::event::Event;
use chrono::Utc;
use influxdb::{Client, Timestamp, WriteQuery};
use std::error::Error;

pub struct InfluxRepository {
    client: Client,
}

impl InfluxRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        let timestamp = Timestamp::Milliseconds(Utc::now().timestamp_millis().try_into()?);

        let mut query = WriteQuery::new(timestamp, "event");
        query = query.add_field("event_type", &event.event.event_type.as_str());

        let result = self.client.query(&query).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
