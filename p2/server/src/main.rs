mod db;
use db::*;

use axum::{
    routing::{Router, get},
    http::Method,
    extract::{State, Path},
    Json,
};
use tower_http::cors::{Any, CorsLayer};
use std::{
    sync::Arc,
    env,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cwd = env::current_dir().unwrap();
    let server = Server::build(cwd).unwrap();
    let app_state = Arc::new(Mutex::new(server));

    //set cors
    let cors = CorsLayer::new()
        .allow_methods(Method::GET)
        .allow_origin(Any);
    let app = Router::new()
        .route("/get/{key}", get(read_value))
        .route("/set/{key}/{value}", get(set_value))
        .route("/set-expire/{key}/{duration}", get(set_expire))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn read_value(
    State(state): State<Arc<Mutex<Server>>>,
    Path(key): Path<u32>,
) -> Json<Option<String>> {
    let re = (*state.lock().await)
        .get(&key);
    Json(re)
}
async fn set_value(
    State(state): State<Arc<Mutex<Server>>>,
    Path((key, value)): Path<(u32, String)>,
) {
    (*state.lock().await)
        .set(key, value);
}
async fn set_expire(
    State(state): State<Arc<Mutex<Server>>>,
    Path((key, duration)): Path<(u32, u32)>,
) {
    (*state.lock().await)
        .expire(key, duration);
}
