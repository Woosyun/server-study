use serde::{Serialize, Deserialize};
use std::{
    time::{self, SystemTime},
    thread
};

#[tokio::main]
async fn main() {
    let id = create_user().await;
    {
        let msg = Message::from(id, "hello I am user1".to_string());
        send_message(1, msg).await;
        let room1 = enter_room(1, id).await;
        println!("user1 read room1:\n{:#?}", room1);
    }
    thread::sleep(time::Duration::from_secs(1));
    {
        let id = create_user().await;
        let msg = Message::from(id, "hello I am user2".to_string());
        send_message(1, msg).await;
        let room1 = enter_room(1, id).await;
        println!("user2 read room1\n{:#?}", room1);
    }
    thread::sleep(time::Duration::from_secs(1));
    {
        let room1 = enter_room(1, id).await;
        println!("user1 read room1:\n{:#?}", room1);
    }
}

fn server() -> &'static str {
    "http://localhost:3000"
}

async fn create_user() -> u64 {
    reqwest::get(server())
        .await.unwrap()
        .json::<u64>()
        .await.unwrap()
}
async fn send_message(room_id: u64, message: Message) {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", server(), room_id);
    client.post(url)
        .json(&message)
        .send()
        .await.unwrap();
}
async fn enter_room(room_id: u64, user_id: u64) -> Vec<Message> {
    let url = format!("{}/{}/{}", server(), room_id, user_id);
    reqwest::get(url)
        .await.unwrap()
        .json::<Vec<Message>>()
        .await.unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    user_id: u64,
    content: String,
    ctime: u64,
}
impl Message {
    pub fn from(user_id: u64, content: String) -> Self {
        let ctime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();
        Self {
            user_id,
            content,
            ctime,
        }
    }
}
