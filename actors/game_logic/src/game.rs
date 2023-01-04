use lazy_static::lazy_static;
use shared::{ClientMessage, BallId};
use std::sync::{Arc,Mutex};
use bevy::prelude::*;
use bevy_warp_wasi::shared::{ConnectionHandle,NetworkEvent};
use crate::systems;
//Remote Addr
lazy_static! {
    pub static ref APP: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));
}
pub fn init(){
   
    let map = APP.clone();
    let mut m = map.lock().unwrap();
    m.add_plugin(bevy_warp_wasi::bevy::WarpServerPlugin::<ClientMessage>::default())
    .add_system(systems::listen_for_events)
    ;
}