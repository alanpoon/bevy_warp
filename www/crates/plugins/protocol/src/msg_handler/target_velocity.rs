use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use shared::*;
pub fn _fn(
    _cmd: &mut Commands,
    set: &mut ParamSet<(Query<(Entity, &BallId, &mut Transform, &mut Velocity), With<BallId>>,)>,
    ball_id: BallId,
    tv: Vec2,
) {
    for (_entity, qball_id, _t, mut v) in set.p0().iter_mut() {
        if ball_id == *qball_id {
            update::target_velocity::velocity(&mut v, tv.clone());
            break;
        }
    }
}
