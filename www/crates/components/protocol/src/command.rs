use serde::{Deserialize, Serialize};
use shared::{UserInfo, ClientMessage};
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
//#[serde(tag = "c")] // stands for code
pub enum Command {
    WS(Vec<u8>),
    StoreLocal(UserInfo),
    #[serde(other)]
    Unknown,
}
#[derive(Clone, Debug, Default)]
pub struct Commands(Vec<Command>);
impl Commands {
    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Command) {
        self.0.push(event);
    }

    pub fn clear(&mut self) {
        self.0.clear();
        self.0.truncate(32);
    }
}
