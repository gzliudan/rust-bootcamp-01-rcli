use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serveing {:?} on port {}", path, addr);
    let state = HttpServeState { path };
    let router = Router::new().route("/*path", get(file_handler).with_state(Arc::new(state)));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // format!("state={:?}, path={}", state.path, path)
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => {
                info!("Read {:?} with {} bytes", p.display(), content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file {:?}: {:?}", p.display(), e.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error reading file {:?}: {:?}", p.display(), e),
                )
            }
        }
    }
}
