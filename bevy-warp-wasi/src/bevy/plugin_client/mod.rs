mod plugin_client;
pub use plugin_client::*;
//#[cfg(target_os = "wasi")]
mod wasi;
pub use wasi::*;
mod www;
pub use www::*;
use futures::{prelude::*};
use eyre::{Result};
use futures::future::{ready};
use bevy::prelude::*;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;
pub struct BoxClient {
    pub clients: Vec<Box<dyn Client + Send + Sync + 'static>>,
}
impl Default for BoxClient {
    fn default() -> Self {
        BoxClient {
            clients: vec![],
        }
    }
}
fn event_receiver(
    rx: impl Stream<Item = Result<Vec<u8>>> + Send + Sync + 'static + Unpin,
) -> impl Stream<Item = Vec<u8>>  + 'static + Unpin {
    rx
        .filter_map(|item|
            ready(match item {
                Ok(ok) => Some(ok),
                Err(err) => {
                    None
                }
            })
        )
}
pub fn set_client(mut client_res: ResMut<Option<BoxClient>>) {
    let mut map = CLIENTS.lock().unwrap();
    for (_k, v) in map.drain() {
        if let Some(ref mut c) = *client_res {
            c.clients.push(v);
        } else {
            let mut bc = BoxClient::default();
            bc.clients = vec![v];
            *client_res = Some(bc);
        }
    }
    if let Some(ref mut _c) = *client_res {}
}
pub trait Client {
    fn sender(&self) -> Box<dyn Sink<Vec<u8>, Error = String> + Send + Sync + Unpin + 'static>;
    fn poll_once(&mut self) -> Option<Vec<Vec<u8>>>;
}
pub type BoxClient2 = Box<dyn Client + Send + Sync + 'static>;

lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<ClientName, BoxClient2>> = Mutex::new(HashMap::new());
}
