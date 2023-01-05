use shared::*;
use crate::messaging_::publish_;
use crate::spawn_::spawn_;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use rand::Rng;
use bevy_warp_wasi::bevy::plugin_server::{WebSocketClient};
use bevy_warp_wasi::shared::ConnectionHandle;
use futures_util::SinkExt;
use tokio::task::spawn;
pub fn _fn (cmd:&mut Commands,set:&mut ParamSet<(
  Query<(&BallId,&BallLabel,&Transform, &mut Velocity)>,
  )>,
  client:&mut ResMut<Vec<WebSocketClient>>,
  ch:&ConnectionHandle,
  game_id:&String,ball_id:&BallId,ball_label:&BallLabel){
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..300) as f32;
    let y = rng.gen_range(0..300) as f32;
    let ball_bundle = BallBundle{
      ball_id:ball_id.clone(),
      ball_label:ball_label.clone(),
      transform:Transform { translation: [x,y,3.0].into(), ..Default::default() },
      global_transform:GlobalTransform::identity(),
      velocity:Velocity::zero(),
      rigid_body:RigidBody::Dynamic,
      locked_axes:LockedAxes::ROTATION_LOCKED,
      interpolated:TransformInterpolation::default()
    };
      spawn_(cmd,ball_bundle.clone());
      let server_message = ServerMessage::Welcome{ball_bundle};
      let server_message = rmp_serde::to_vec(&server_message).unwrap();
        for c in client.iter(){
            let mut b = c.sender();
            let server_message_c = server_message.clone();
            b.send(server_message_c);

        }
    
      let mut ball_bundles =vec![];
      let mut query = set.p0();
      for (gball_id,ball_label,transform,velocity) in query.iter(){
        if gball_id.0!=ball_id.0{//don't send yourself
          ball_bundles.push(BallBundle{ball_id:gball_id.clone(),ball_label:ball_label.clone(),
            transform:transform.clone(),global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),rigid_body:RigidBody::Dynamic,
            locked_axes:LockedAxes::ROTATION_LOCKED,
            interpolated:TransformInterpolation::default()});
        }
      }

      let channel_message_back = ServerMessage::GameState{ball_bundles:ball_bundles};
      for c in client.iter(){
        if &c.connection_handle()==ch{
          
          let data = rmp_serde::to_vec(&channel_message_back.clone()).unwrap();
          c.sender().send(data);
          break;
        }          
      }
      
}