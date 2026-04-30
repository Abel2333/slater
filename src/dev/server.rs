use axum::Router;
use axum::routing::get_service;
use tokio::signal;
use tower_http::services::ServeDir;

use crate::config::SiteConfig;
use crate::error::Result;

pub async fn serve(config: &SiteConfig) -> Result<()> {
    crate::render::builder::build_site(config)?;
    let listener =
        tokio::net::TcpListener::bind((config.dev.host.as_str(), config.dev.port)).await?;
    println!("Serving at http://{}:{}", config.dev.host, config.dev.port);

    let app = Router::new().fallback_service(get_service(ServeDir::new(&config.output_dir)));

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
