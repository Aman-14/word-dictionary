use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use serde::Deserialize;
use storage::Storage;

mod storage;

struct AppState {
    storage: Storage,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        storage: Storage::new(),
    });
    let app = Router::new()
        .route("/", routing::get(index))
        .route("/definition", routing::get(get_definition))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    return "Dictionary is running";
}

#[derive(Deserialize)]
struct Word {
    word: String,
}

#[debug_handler]
async fn get_definition(
    Query(Word { word }): Query<Word>,
    app_state: State<Arc<AppState>>,
) -> Result<String, StatusCode> {
    return app_state
        .storage
        .get_definition(word)
        .map_or(Err(StatusCode::NOT_FOUND), Ok);
}