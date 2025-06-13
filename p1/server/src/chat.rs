#![allow(unused)]

use std::{
    collections::HashMap,
    sync::{RwLock, Arc},
};
use serde::{Serialize, Deserialize};

pub type UserId = u64;
pub type RoomId = u64;
pub type Time = u64;

fn current_time() -> u64 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap();
    now.as_secs()
}

pub struct Room {
    messages: Vec<Message>,
    last_read: HashMap<UserId, Time>,
}
impl Room {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            last_read: HashMap::new()
        }
    }
    pub fn read_messages(&mut self, user_id: UserId) -> Vec<Message> {
        let mut last_read = self.last_read
            .entry(user_id)
            .or_insert_with(|| 0);

        //48h
        let max = 60*60*48;
        self.messages = self.messages.iter()
            .filter_map(|msg| {
                if msg.ctime >= max {
                    Some(msg)
                } else {
                    None
                }
            })
            .cloned()
            .collect();
            

        let mut result = Vec::new();
        for message in self.messages.iter() {
            if *last_read < message.ctime {
                result.push(message.clone());
            }
        }

        *last_read = current_time();
        
        result
    }
    pub fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    user_id: UserId,
    ctime: Time,
    content: String,
}
