use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tokio::signal;

use crate::domain::aggregates::metrics::{AppMetrics, Metrics};
use crate::infrastructure::http::handlers::capture::capture_endpoint;
use crate::infrastructure::http::handlers::metrics::metrics_endpoint;

mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metrics = Metrics::new();
    let state = web::Data::new(AppMetrics {
        metrics: Mutex::new(metrics),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/capture", web::post().to(capture_endpoint))
            .route("/metrics", web::get().to(metrics_endpoint))
    })
    .bind("127.0.0.1:3000")?
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
