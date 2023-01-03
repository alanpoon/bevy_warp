use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_warp_wasi::shared::{ConnectionHandle};
use shared::*;
use crate::client_message_handlers;
pub fn listen_for_events(mut cmd:Commands, mut evs: EventReader<(ConnectionHandle,ClientMessage)>,
    
    mut set: ParamSet<(
        Query<(&BallId,&BallLabel,&Transform, &Velocity)>,
        // also access the whole world ... why not
        //&World,
    )>,) {
    for  (ch,cm) in evs.iter() {
        println!("received DummyEvent from  {:?} {:?}", ch,cm);
        match cm{
            ClientMessage::TargetVelocity{game_id,ball_id,target_velocity}=>{
                //client_message_handlers::target_velocity_handler::_fn(set,game_id,ball_id,target_velocity);  
            }
            ClientMessage::Welcome{game_id,ball_id,ball_label}=>{
                client_message_handlers::welcome_handler::_fn(&mut cmd, &mut set,game_id,ball_id,ball_label);
            }
            _=>{

            }
        }
    }
}