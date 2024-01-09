use std::error::Error;

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::sql::PostgresRepository;

pub struct EventsProcessor {
    repository: PostgresRepository,
}

impl EventsProcessor {
    pub fn new(repository: PostgresRepository) -> Self {
        Self { repository }
    }

    pub fn process(&self, event: Event) -> Result<(), Box<dyn Error>> {
        match event {}
    }
}
