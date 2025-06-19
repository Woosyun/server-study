use serde::{Serialize, Deserialize};
use std::{
    thread,
    time::{Duration, SystemTime},
};


#[tokio::main]
async fn main() {
    let user_1 = create_user().await;
    {
        let message = Message::from(user_1, "hello I am user_1");
        send_message(0, message).await;
        let enter = enter_room(0, user_1).await;
        println!("{} entered room {}: {:#?}", user_1, 0, enter);
    }
    let user_2 = create_user().await;
    thread::sleep(Duration::from_secs(1));
    {
        let message = Message::from(user_2, "hello I am user_2");
        send_message(0, message).await;
        let enter = enter_room(0, user_2).await;
        println!("{} entered room {}: {:#?}", user_2, 0, enter);
    }
    thread::sleep(Duration::from_secs(1));
    {
        let enter = enter_room(0, user_1).await;
        println!("{} entered room {}: {:#?}", user_1, 0, enter);
    }
}

async fn create_user() -> u64 {
    reqwest::get("http://localhost:3000")
        .await.unwrap()
        .json::<u64>()
        .await.unwrap()
}
#[derive(Serialize)]
struct EnterRoomQuery {
    room_id: u64,
    user_id: u64,
}
async fn enter_room(room_id: u64, user_id: u64) -> Vec<Message> {
    let q = EnterRoomQuery {
        room_id,
        user_id,
    };

    let client = reqwest::Client::new();
    client.get("http://localhost:3000/room")
        .query(&q)
        .send()
        .await.unwrap()
        .json::<Vec<Message>>()
        .await.unwrap()
}
#[derive(Serialize)]
struct SendMessageQuery {
    room_id: u64
}
async fn send_message(room_id: u64, message: Message) {
    let q = SendMessageQuery {
        room_id
    };

    let client = reqwest::Client::new();
    client.post("http://localhost:3000/room")
        .query(&q)
        .json(&message)
        .send()
        .await.unwrap();
}


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    user_id: u64,
    content: String,
    ctime: u64,
}
impl Message {
    fn from(user_id: u64, content: &str) -> Self {
        let ctime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();
        Self {
            user_id,
            content: content.to_string(),
            ctime,
        }
    }
}
