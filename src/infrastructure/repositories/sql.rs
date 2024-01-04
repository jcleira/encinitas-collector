use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

use crate::domain::aggregates::event::Event;
use crate::infrastructure::repositories::sql_models::{events, requests, responses};

use super::sql_models::db_event_from_aggregate;
use super::sql_models::db_request_from_aggregate;
use super::sql_models::db_response_from_aggregate;

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

    // Example of a method using the pool
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.pool.get()
    }

    pub fn create_event(&self, event: &Event) -> QueryResult<usize> {
        let db_event = db_event_from_aggregate(&event.event);
        let mut conn = self.get_connection().expect("Failed to get connection");
        diesel::insert_into(events::table)
            .values(db_event)
            .execute(&mut conn)
    }

    pub fn create_request(&self, event: &Event) -> QueryResult<usize> {
        let db_request = db_request_from_aggregate(event.request.as_ref().unwrap());
        let mut conn = self.get_connection().expect("Failed to get connection");
        diesel::insert_into(requests::table)
            .values(db_request)
            .execute(&mut conn)
    }

    pub fn create_response(&self, event: &Event) -> QueryResult<usize> {
        let mut conn = self.get_connection().expect("Failed to get connection");
        let db_response = db_response_from_aggregate(event.response.as_ref().unwrap());
        diesel::insert_into(responses::table)
            .values(db_response)
            .execute(&mut conn)
    }
}
