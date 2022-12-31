use std::collections::HashMap;
use std::sync::{
    Arc,
};
use tokio::sync::{RwLock};
use tokio::sync::mpsc;
use warp::ws::{Message};
pub type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;