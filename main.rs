use axum::{routing::any, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback(any(handler));
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.expect("Gagal bind port");
        if let Ok(std_listener) = listener.std_lookup().and_then(|l| l.try_clone()) {
    }

    println!("on: {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "ok!"
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
        _ = terminate => {},
    }
}
