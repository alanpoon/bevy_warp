use bevy::prelude::*;
use shared::*;
pub fn spawn_(w: &mut Commands,ball_bundle:BallBundle){
  w.spawn_batch(
    vec![ball_bundle]
  );
}