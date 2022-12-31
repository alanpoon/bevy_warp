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
    mut events: ResMut<Vec<NetworkEvent>>,
    mut sink: EventWriter<(ConnectionHandle,ClientMessage)>,){
    //mut sink: EventWriter<NetworkEvent>,){
    for ev in events.drain(..) {
        match ev{
            NetworkEvent::Message(ch,data )=>{
                if data==vec![2]{
                    sink.send((ch,ClientMessage::TargetVelocity { game_id:String::from("ss"), ball_id: BallId(2), target_velocity: Vec2::new(0.0,0.0) }));
                }
            }
            _=>{

            }
        }
    }
}

pub fn init(){
   
    let map = APP.clone();
    let mut m = map.lock().unwrap();
    m.add_plugin(bevy_warp_wasi::bevy::WarpPlugin::<ClientMessage>::default())
    .add_system(systems::listen_for_events)
    .add_system(handle_network_events);
}