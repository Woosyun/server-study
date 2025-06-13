use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    {
        let user1 = create_user().await;
        send_message(user1, 0, "hello world".to_string()).await;
        send_message(user1, 0, "I'm user1".to_string()).await;
    }


    {
        let user2 = create_user().await;
        let res2 = enter_room(user2, 0).await;
        println!("user {} entered room {}: {:#?}", user2, 0, res2);
    }
}

#[derive(Serialize)]
struct SendMessagePayLoad {
    room_id: u64,
    message: Message,
}
impl SendMessagePayLoad {
    fn new(room_id: u64, message: Message) -> Self {
        Self {
            room_id,
            message,
        }
    }
}
async fn send_message(user_id: u64, room_id: u64, content: String) {
    let msg = Message::new(user_id, content);
    let p = SendMessagePayLoad::new(room_id, msg);
    let client = reqwest::Client::new();
    client.post("http://localhost:3000/send-message")
        .json(&p)
        .send()
        .await.unwrap();
}

async fn create_user() -> u64 {
    let id = reqwest::get("http://localhost:3000/create-user")
        .await.unwrap()
        .json::<u64>()
        .await.unwrap();

    id
}

async fn enter_room(user_id: u64, room_id: u64) -> Vec<Message> {
    let p = EnterRoomPayLoad::new(user_id, room_id);
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000/enter-room")
        .json(&p)
        .send()
        .await.unwrap()
        .json::<Vec<Message>>().await.unwrap();

    res
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    user_id: u64,
    ctime: u64,
    content: String,
}
impl Message {
    fn new(user_id: u64, content: String) -> Self {
        use std::time::SystemTime;
        let ctime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();
        Self {
            user_id,
            ctime,
            content,
        }
    }
}
#[derive(Serialize)]
struct EnterRoomPayLoad {
    user_id: u64,
    room_id: u64,
}
impl EnterRoomPayLoad {
    fn new(user_id: u64, room_id: u64) -> Self {
        Self {
            user_id,
            room_id,
        }
    }
}
