use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use log::{info, warn};
use std::convert::Infallible;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower::ServiceExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

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

    // Path to index.html for SPA fallback
    let index_path = frontend_dir.join("index.html");

    // Create a service that serves files from the frontend directory
    // with index.html as fallback for SPA routing
    let serve_dir = ServeDir::new(&frontend_dir)
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new(&index_path));

    let app = Router::new()
        .nest_service("/", serve_dir)
        .layer(cors);

    // Try ports 1420-1430 to find an available one
    let mut port = 1420;
    let listener = loop {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("Static file server starting on http://{}", addr);
                info!("Serving frontend from: {}", frontend_dir.display());
                info!("Web browser can access the app at http://localhost:{}", port);
                break listener;
            }
            Err(e) => {
                if port < 1430 {
                    warn!("Port {} is in use, trying next port: {}", port, e);
                    port += 1;
                } else {
                    warn!("Could not bind static file server to any port 1420-1430: {}", e);
                    return Err(format!("Failed to bind static file server: {}", e).into());
                }
            }
        }
    };

    axum::serve(listener, app).await?;

    Ok(())
}
