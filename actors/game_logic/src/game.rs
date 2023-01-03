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
pub fn handle_network_events(
    mut stream: EventReader<(ConnectionHandle,ClientMessage)>,){
    for client_message in stream.iter(){
        match client_message{
            Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
              let map = APP.clone();
              client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
            }
            Ok(ClientMessage::Welcome{game_id,ball_id,ball_label})=>{
              let map = APP.clone();
              client_message_handlers::welcome_handler::_fn(map,game_id,ball_id,ball_label).await;
            }
            
            Err(e)=>{
              info!("client_message err {:?}",e);
            }
          }
    }
}

pub fn init(){
   
    let map = APP.clone();
    let mut m = map.lock().unwrap();
    m.add_plugin(bevy_warp_wasi::bevy::WarpServerPlugin::<ClientMessage>::default())
    .add_system(systems::listen_for_events)
    .add_system(handle_network_events);
}
pub fn init2()->Arc<Mutex<App>>{
  let app =  Arc::new(Mutex::new(App::new()));
  let mut m = map.lock().unwrap();
  m.add_plugin(bevy_warp_wasi::bevy::WarpServerPlugin::<ClientMessage>::default())
  .add_system(systems::listen_for_events)
  .add_system(handle_network_events);
  app
}