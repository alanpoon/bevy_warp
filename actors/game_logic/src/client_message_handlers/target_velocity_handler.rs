use shared::*;
use crate::messaging_::publish_;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
pub fn _fn (set:&mut ParamSet<(
  Query<(&BallId,&BallLabel,&Transform, &mut Velocity)>,
  )>,_game_id:&String,ball_id:&BallId,target_velocity:&Vec2){
  for (gball_id,_,transform,mut velocity) in set.p0().iter_mut(){
    if gball_id.0 ==ball_id.0{
      update::target_velocity::velocity(&mut velocity, target_velocity.clone());
      
    }
  }
  
}