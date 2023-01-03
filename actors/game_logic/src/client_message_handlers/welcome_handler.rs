use shared::*;
use crate::messaging_::publish_;
use crate::spawn_::spawn;
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use rand::Rng;
pub async fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,ball_label:BallLabel)-> RpcResult<()>{
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..300);
    let y = rng.gen_range(0..300);
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      ball_label:ball_label.clone(),
      transform:Transform { translation: [x,y,3.0].into(), ..Default::default() },
      global_transform:GlobalTransform::identity(),
      velocity:Velocity::zero(),
      rigid_body:RigidBody::Dynamic,
      locked_axes:LockedAxes::ROTATION_LOCKED,
      interpolated:TransformInterpolation::default()
    };
    {
      let guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          poisoned.into_inner()
        },
      };
      let mut app = guard;
      //info_(format!("welcome {:?}",ball_bundle.clone()));
      spawn(&mut app.world,ball_bundle.clone());
      let server_message = ServerMessage::Welcome{ball_bundle};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          publish_(b);
        }
        _=>{}
      }
      let mut ball_bundles =vec![];
      let mut query = app.world.query::<(&BallId,&BallLabel,&Transform, &Velocity)>();
      for (gball_id,ball_label,transform,velocity) in query.iter(&app.world){
        if gball_id.0!=ball_id.0{//don't send yourself
          ball_bundles.push(BallBundle{ball_id:gball_id.clone(),ball_label:ball_label.clone(),
            transform:transform.clone(),global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),rigid_body:RigidBody::Dynamic,
            locked_axes:LockedAxes::ROTATION_LOCKED,
            interpolated:TransformInterpolation::default()});
        }
      }

      let channel_message_back = ServerMessage::GameState{ball_bundles:ball_bundles};
      app.world.send_event();
    }
    Ok(())
}