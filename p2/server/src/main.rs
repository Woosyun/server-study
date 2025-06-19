mod db;
use db::*;

use std::sync::{Arc, Mutex};
use axum::{
    http::Method,
    routing::{Router, get, post},
    extract::{State, Path},
    Json,
};
use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let workdir = std::env::current_dir().unwrap();
    let server = Server::build(workdir);
    let server = Arc::new(Mutex::new(server));

    let app = Router::new()
        .route("/get/{key}", get(read))
        .route("/set/{key}", post(write))
        .route("/expires/{key}", post(expires))
        .with_state(server)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn read(
    State(server): State<Arc<Mutex<Server>>>,
    Path(key): Path<u64>,
) -> Json<Option<Value>> {
    let re = (*server.lock().unwrap())
        .get(key);
    Json(re)
}
async fn write(
    State(server): State<Arc<Mutex<Server>>>,
    Path(key): Path<u64>,
    value: String,
) {
    (*server.lock().unwrap())
        .set(key, value);
}

async fn expires(
    State(server): State<Arc<Mutex<Server>>>,
    Path(key): Path<u64>,
    Json(duration): Json<u64>,
) {
    (*server.lock().unwrap())
        .expires(key, duration);
}
