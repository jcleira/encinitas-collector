use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::sql_models::{
    events, requests, responses, DBEvent, DBRequest, DBResponse,
};

pub struct PostgresRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Self { pool }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.pool.get()
    }

    pub fn create_event(&self, event: &Event) -> QueryResult<usize> {
        let db_event = DBEvent::from_aggregate(event);
        let mut conn = self.get_connection().expect("Failed to get connection");
        diesel::insert_into(events::table)
            .values(db_event)
            .execute(&mut conn)
    }

    pub fn create_request(&self, event: &Event) -> QueryResult<usize> {
        let db_request = DBRequest::from_aggregate(event.request.as_ref().unwrap());
        let mut conn = self.get_connection().expect("Failed to get connection");
        diesel::insert_into(requests::table)
            .values(db_request)
            .execute(&mut conn)
    }

    pub fn create_response(&self, event: &Event) -> QueryResult<usize> {
        let mut conn = self.get_connection().expect("Failed to get connection");
        let db_response = DBResponse::from_aggregate(event.response.as_ref().unwrap());
        diesel::insert_into(responses::table)
            .values(db_response)
            .execute(&mut conn)
    }

    pub fn get_event(&self, event_id: &str) -> QueryResult<Event> {
        let mut conn = self.get_connection().expect("Failed to get connection");
        // Assuming `events::id` is the correct field and `DBEvent` is your Diesel model
        events::table
            .filter(events::id.eq(event_id))
            .first::<DBEvent>(&mut conn)
            // Assuming `to_aggregate` converts a `DBEvent` to your domain `Event`
            .map(|db_event| db_event.to_aggregate())
            // Handle possible conversion errors
            .map_err(|e| e.into())
    }
}
