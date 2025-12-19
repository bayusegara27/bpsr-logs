use axum::{routing::get_service, Router};
use log::{info, warn};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

/// Start a static file server on port 1420 to serve the built frontend
/// This allows web browser access via tunneling (cloudflared, ngrok, etc.)
pub async fn start_static_server(
    frontend_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check if the frontend directory exists
    if !frontend_dir.exists() {
        warn!(
            "Frontend directory does not exist: {}. Static server will not start.",
            frontend_dir.display()
        );
        return Ok(());
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create a service that serves files from the frontend directory
    let serve_dir = ServeDir::new(&frontend_dir).append_index_html_on_directories(true);

    let app = Router::new()
        .fallback_service(get_service(serve_dir))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 1420));
    info!("Static file server starting on http://{}", addr);
    info!("Serving frontend from: {}", frontend_dir.display());
    info!("Web browser can access the app at http://localhost:1420");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
