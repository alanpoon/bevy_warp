use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;
pub fn _fn(
    cmd: &mut Commands,
    ball_bundle:BallBundle,
) {
    cmd.spawn_bundle(ball_bundle);
}
