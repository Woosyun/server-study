mod chat;
use chat::*;

use serde::{Serialize, Deserialize};
use std::{
    sync::{Arc, Mutex},
    collections::HashMap
};
use axum::{
    routing::{Router, get, post},
    http::Method,
    Json,
    extract::State,
};
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    rooms: HashMap<RoomId, Room>,
    users: u64,
}
impl AppState {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            users: 0
        }
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app_state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/create-user", get(create_user))
        .route("/enter-room", post(enter_room))
        .route("/send-message", post(send_message))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(state): State<Arc<Mutex<AppState>>>
) -> Json<u64> {
    (*state.lock().unwrap()).users += 1;
    
    let new_user_id = (*state.lock().unwrap()).users;
    Json(new_user_id)
}

#[derive(Serialize, Deserialize, Clone)]
struct EnterRoomPayLoad {
    user_id: u64,
    room_id: u64,
}
async fn enter_room(
    State(state): State<Arc<Mutex<AppState>>>,
    p: Json<EnterRoomPayLoad>,
) -> Json<Vec<chat::Message>> {
    let room_id = p.room_id;
    let user_id = p.user_id;

    println!("user {} entered {}", user_id, room_id);

    let messages = (*state.lock().unwrap())
        .rooms
        .entry(room_id)
        .or_insert_with(|| Room::new())
        .read_messages(user_id);

    Json(messages)
}

#[derive(Deserialize)]
struct SendMessagePayLoad {
    room_id: u64,
    message: Message,
}
async fn send_message(
    State(state): State<Arc<Mutex<AppState>>>,
    p: Json<SendMessagePayLoad>,
) {
    let room_id = p.room_id;
    let message = p.message.clone();

    println!("message: {:?} sent to room {}", message, room_id);

    (*state.lock().unwrap())
        .rooms
        .entry(room_id)
        .or_insert(Room::new())
        .send_message(message);
}
