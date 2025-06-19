mod chat;
use chat::*;

use serde::Deserialize;
use axum::{
    routing::{Router, get, post},
    http::Method,
    extract::{State, Query},
    Json
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let server = Arc::new(Mutex::new(Server::new()));

    let app = Router::new()
        .route("/", get(create_user))
        .route("/room", get(enter_room))
        .route("/room", post(send_message))
        .with_state(server)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn create_user(
    State(server): State<Arc<Mutex<Server>>>,
) -> Json<UserId> {
    let re = (*server.lock().unwrap())
        .create_user();

    println!("new user: {}", re);

    Json(re)
}

#[derive(Deserialize)]
struct EnterRoomQuery {
    room_id: RoomId,
    user_id: UserId,
}
async fn enter_room(
    State(server): State<Arc<Mutex<Server>>>,
    Query(q): Query<EnterRoomQuery>
) -> Json<Vec<Message>> {
    let re = (*server.lock().unwrap())
        .enter_room(q.room_id, q.user_id);

    Json(re)
}

#[derive(Deserialize)]
struct SendMessageQuery {
    room_id: RoomId,
}
async fn send_message(
    State(server): State<Arc<Mutex<Server>>>,
    Query(q): Query<SendMessageQuery>,
    Json(message): Json<Message>,
)  {
    println!("message {:#?} sent to room {}", &message, q.room_id);
    (*server.lock().unwrap())
        .send_message(q.room_id, message)
}
