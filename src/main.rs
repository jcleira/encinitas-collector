use actix_web::{web, App, HttpServer};
use influxdb::Client;
use std::sync::Arc;
use tokio::signal;

use crate::config::Config;
use crate::domain::services::events_creator::EventsCreator;
use crate::infrastructure::http::handlers::capture::CaptureEndpoint;
use crate::infrastructure::repositories::influx::InfluxRepository;

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

    let capture_endpoint = CaptureEndpoint::new(EventsCreator::new(influx_repository));
    let capture_endpoint = Arc::new(capture_endpoint);

    let server = HttpServer::new(move || {
        let capture_endpoint_clone = Arc::clone(&capture_endpoint);
        App::new().route(
            "/capture",
            web::post().to(move |http_event| {
                let capture_endpoint = capture_endpoint_clone.clone();
                async move { capture_endpoint.endpoint(http_event).await }
            }),
        )
    })
    .bind("0.0.0.0:3000")?
    .run();

    let server_handle = server.handle();

    tokio::select! {
        _ = server => {
            println!("Server stopped");
        }
        _ = signal::ctrl_c() => {
            println!("Ctrl-C received, stopping server...");
            server_handle.stop(true).await;
            println!("Server has been stopped");
        }
    }

    Ok(())
}
