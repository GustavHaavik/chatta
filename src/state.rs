use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Message {
    pub text: String,
    pub user: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

pub type RoomStore = HashMap<String, VecDeque<Message>>;

#[derive(Default)]
pub struct MessageStore {
    pub messages: RwLock<RoomStore>,
}

impl MessageStore {
    pub async fn add_message(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_string()).or_default();
        messages.push_back(message);
        messages.truncate(100);
    }

    pub async fn get_messages(&self, room: &str) -> Vec<Message> {
        let messages = self.messages.read().await;
        messages.get(room).cloned().unwrap_or_default().into()
    }
}
