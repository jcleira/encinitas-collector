use std::error::Error;

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::influx::InfluxRepository;

pub struct EventsCreator {
    repository: InfluxRepository,
}

impl EventsCreator {
    pub fn new(repository: InfluxRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, event: &Event) -> Result<(), Box<dyn Error>> {
        return self.repository.create(event).await;
    }
}
