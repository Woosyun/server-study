mod chat;
use chat::*;

use axum::{
    routing::{get, Router, post},
    http::Method,
    extract::{State, Path},
    Json,
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let server = Arc::new(Mutex::new(Server::new()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(create_user))
        .route("/{room_id}", post(send_message))
        .route("/{room_id}/{user_id}", get(enter_room))
        .with_state(server)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(server): State<Arc<Mutex<Server>>>,
) -> Json<UserId> {
    Json((*server.lock().unwrap())
        .create_user())
}

async fn send_message(
    State(server): State<Arc<Mutex<Server>>>,
    Path(room_id): Path<RoomId>,
    Json(message): Json<Message>,
) {
    (*server.lock().unwrap())
        .send_message(room_id, message)
}
async fn enter_room(
    State(server): State<Arc<Mutex<Server>>>,
    Path((room_id, user_id)): Path<(RoomId, UserId)>,
) -> Json<Vec<Message>> {
    let result = (*server.lock().unwrap())
        .enter_room(room_id, user_id);
    println!("{} entered room {}: {:#?}", user_id, room_id, result);
    Json(result)
}
