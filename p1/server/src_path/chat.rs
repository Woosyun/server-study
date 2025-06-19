use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type RoomId = u64;
pub type UserId = u64;
pub type Time = u64;

#[derive(Serialize, Deserialize)]
pub struct Server {
    rooms: HashMap<RoomId, Room>,
    number_of_user: u64,
}
impl Server {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            number_of_user: 0,
        }
    }
    pub fn create_user(&mut self) -> UserId {
        self.number_of_user += 1;
        self.number_of_user
    }
    pub fn enter_room(&mut self, room_id: RoomId, user_id: UserId) -> Vec<Message> {
        self.rooms
            .entry(room_id)
            .or_insert_with(|| Room::new())
            .read_messages(user_id)
    }
    pub fn send_message(&mut self, room_id: RoomId, message: Message) {
        self.rooms
            .entry(room_id)
            .or_insert_with(|| Room::new())
            .send_message(message)
    }
}
#[derive(Serialize, Deserialize)]
pub struct Room {
    messages: Vec<Message>,
    last_read: HashMap<UserId, Time>,
}
impl Room {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            last_read: HashMap::new(),
        }
    }
    pub fn read_messages(&mut self, user_id: UserId) -> Vec<Message> {
        use std::time::SystemTime;

        let last_read = self.last_read
            .entry(user_id)
            .or_insert_with(|| 0);

        let messages = self.messages
            .iter()
            .filter_map(|message| {
                if (*last_read) < message.ctime {
                    Some(message.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        (*last_read) = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

        messages
    }
    pub fn send_message(&mut self, message: Message) {
        self.messages.push(message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    user_id: UserId,
    content: String,
    ctime: Time,
}
