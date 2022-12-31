use serde::{Deserialize, Serialize};
use shared::*;
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    ServerMessage(ServerMessage),
    BevyWeb(serde_json::Value),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct Events(Vec<Event>);

impl Events {
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Event) {
        self.0.push(event);
    }
    pub fn retain<F: FnMut(&Event) -> bool>(&mut self, f: F) {
        self.0.retain(f);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn truncate(&mut self) {
        self.0.truncate(32);
    }
}
