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
    info!("🚀 Attempting to start static file server...");
    
    // Check if the frontend directory exists
    if !frontend_dir.exists() {
        warn!("📁 Frontend directory does not exist: {}", frontend_dir.display());
        warn!("ℹ️  Static server will not start - this is normal in development mode");
        warn!("💡 In development, Vite dev server runs on port 1420 instead");
        warn!("📦 For production, ensure 'build' directory is bundled with the app");
        return Ok(());
    }

    info!("📁 Found frontend directory: {}", frontend_dir.display());

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
    info!("🌐 Looking for available port for static file server...");
    let mut port = 1420;
    let listener = loop {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("📡 Attempting to bind static file server to port {}...", port);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("✅ Static file server successfully started on http://{}", addr);
                info!("🌐 Web browser can access the app at http://localhost:{}", port);
                info!("📂 Serving frontend from: {}", frontend_dir.display());
                info!("🔧 SPA fallback enabled (all routes → index.html)");
                info!("🌍 CORS enabled for tunnel access (cloudflared, ngrok, etc.)");
                break listener;
            }
            Err(e) => {
                if port < 1430 {
                    warn!("⚠️  Port {} is already in use ({}), trying port {}...", port, e, port + 1);
                    port += 1;
                } else {
                    warn!("❌ Could not bind static file server to any port 1420-1430: {}", e);
                    warn!("💡 Please close any applications using these ports and restart");
                    return Err(format!("Failed to bind static file server: {}", e).into());
                }
            }
        }
    };

    info!("🎯 Static file server is ready and serving files");
    axum::serve(listener, app).await?;

    Ok(())
}
