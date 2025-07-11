use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type RoomId = u64;
pub type UserId = u64;
pub type Time = u64;

pub struct Server {
    rooms: HashMap<RoomId, Room>,
    users: u64,
}
impl Server {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            users: 0,
        }
    }

    pub fn create_user(&mut self) -> UserId {
        self.users += 1;
        self.users
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

struct Room {
    messages: Vec<Message>,
    last_read: HashMap<UserId, Time>
}
impl Room {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            last_read: HashMap::new()
        }
    }
    fn read_messages(&mut self, user_id: UserId) -> Vec<Message> {
        let last_read = self.last_read
            .entry(user_id)
            .or_insert(0);

        let result = self.messages
            .iter()
            .filter_map(|message| {
                if (*last_read) < message.ctime {
                    Some(message.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        (*last_read) = result.get(result.len()-1).unwrap()
            .ctime;

        result
    }
    fn send_message(&mut self, message: Message) {
        self.messages
            .push(message)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    user_id: UserId,
    content: String,
    ctime: Time,
}
