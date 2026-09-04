use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use iml_core_lib::Arena;
#[cfg(not(debug_assertions))]
use axum::http::header;

#[cfg(not(debug_assertions))]
use mime_guess::from_path;
use reqwest::Client;
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::llm::rewrite_node;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Asset;

pub struct AppState {
    pub arena: Arc<Mutex<Arena>>,
    pub client: Client,
}

#[derive(Deserialize)]
pub struct TranslateRequest {
    pub node_index: usize,
    pub updated_text: String,
}

pub async fn get_ast(State(state): State<Arc<AppState>>) -> Json<Arena> {
    let arena = state.arena.lock().await;
    Json(arena.clone())
}

pub async fn translate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TranslateRequest>,
) -> impl IntoResponse {
    let arena = {
        state.arena.lock().await.clone()
    };

    match rewrite_node(&state.client, &arena, payload.node_index, &payload.updated_text).await {
        Ok(new_arena) => {
            let mut master_arena = state.arena.lock().await;
            *master_arena = new_arena.clone();
            (StatusCode::OK, Json(new_arena)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[cfg(not(debug_assertions))]
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    
    if path.is_empty() {
        path = "index.html".to_string();
    }
    
    match Asset::get(path.as_str()) {
        Some(content) => {
            let mime = from_path(&path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path != "index.html" {
                match Asset::get("index.html") {
                    Some(content) => {
                        let mime = from_path("index.html").first_or_octet_stream();
                        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                    }
                    None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
                }
            } else {
                (StatusCode::NOT_FOUND, "404 Not Found").into_response()
            }
        }
    }
}

#[cfg(debug_assertions)]
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path();
    let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();
    let url = format!("http://localhost:5173{}{}", path, query);
    
    let client = Client::new();
    let res = match client.get(&url).send().await {
        Ok(r) => r,
        Err(_) => return (StatusCode::BAD_GATEWAY, "Vite server not running").into_response(),
    };
    
    let mut builder = Response::builder().status(res.status());
    for (k, v) in res.headers() {
        builder = builder.header(k, v);
    }
    
    match res.bytes().await {
        Ok(bytes) => builder.body(axum::body::Body::from(bytes)).unwrap().into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error reading proxy response").into_response(),
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/ast", get(get_ast))
        .route("/translate", post(translate))
        .fallback(static_handler)
        .layer(cors)
        .with_state(state)
}
