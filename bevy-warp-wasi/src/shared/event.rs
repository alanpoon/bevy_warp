use thiserror::Error as TError;
use super::ConnectionHandle;

#[derive(TError, Debug)]
pub enum NetworkError {}

#[derive(Debug,Clone)]
pub enum NetworkEvent {
    Connected(ConnectionHandle),
    Disconnected(ConnectionHandle),
    Message(ConnectionHandle, Vec<u8>),
    //Error(Option<ConnectionHandle>, anyhow::Error),
}
use std::sync::{Arc,Mutex};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref EVENTS: Arc<Mutex<Vec<NetworkEvent>>> = Arc::new(Mutex::new(vec![]));
}