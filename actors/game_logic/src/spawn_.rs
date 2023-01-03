use bevy::prelude::*;
use shared::*;
pub fn spawn(w: &mut World,ball_bundle:BallBundle){
  w.spawn_batch(
    vec![ball_bundle]
  );
}