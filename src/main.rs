use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use influxdb::Client;
use std::sync::Arc;
use tokio::signal;

use crate::config::Config;
use crate::domain::services::events_creator::EventsCreator;
use crate::domain::services::events_processor::EventsProcessor;
use crate::infrastructure::http::handlers::capture::CaptureEndpoint;
use crate::infrastructure::redis::consumers::events::EventsConsumer;
use crate::infrastructure::redis::producers::events::EventsProducer;
use crate::infrastructure::repositories::influx::InfluxRepository;
use crate::infrastructure::repositories::redis::RedisRepository;
use crate::infrastructure::repositories::sql::PostgresRepository;

mod config;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let cfg = Config::new().expect("Failed to load config");

    println!("Starting server with influxdb in : {:?}", cfg.influxdb_url);

    let influx_repository = InfluxRepository::new(
        Client::new(cfg.influxdb_url, cfg.influxdb_db).with_token(cfg.influxdb_token),
    );

    let postgres_repository = PostgresRepository::new(&cfg.database_url);

    let events_producer = EventsProducer::new(RedisRepository::new(&cfg.redis_url));

    let capture_endpoint = CaptureEndpoint::new(EventsCreator::new(
        influx_repository,
        postgres_repository,
        events_producer,
    ));
    let capture_endpoint = Arc::new(capture_endpoint);

    let events_consumer_redis_url = cfg.redis_url.clone();
    let events_consumer_postgres_url = cfg.database_url.clone();
    let events_consumer = tokio::spawn(async move {
        EventsConsumer::new(
            RedisRepository::new(&events_consumer_redis_url),
            EventsProcessor::new(PostgresRepository::new(
                events_consumer_postgres_url.as_str(),
            )),
        )
        .consume()
        .await
    });

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600); // The max_age is optional

        let capture_endpoint_clone = Arc::clone(&capture_endpoint);
        App::new().wrap(cors).route(
            "/capture",
            web::post().to(move |http_event| {
                let capture_endpoint = capture_endpoint_clone.clone();
                async move { capture_endpoint.endpoint(http_event).await }
            }),
        )
    })
    .bind("0.0.0.0:3001")?
    .run();

    let server_handle = server.handle();

    tokio::select! {
        _ = server => {
            println!("Server stopped");
        }
        _ = events_consumer => {
            println!("Consumer task completed or stopped");
        }
        _ = signal::ctrl_c() => {
            println!("Ctrl-C received, stopping server...");
            server_handle.stop(true).await;
            println!("Server has been stopped");
        }
    }

    Ok(())
}
