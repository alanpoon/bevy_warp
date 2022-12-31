use bevy::{prelude::*, render::camera::Camera};
use shared::*;

// A simple camera system for moving and zooming the camera.
pub fn move_with_local_player(
    mut commands: ResMut<protocol::Commands>,
    mut local_user_info: ResMut<LocalUserInfo>,
    ball_query: Query<(&BallId, &Transform), (With<BallId>, Without<Camera>)>,
    mut query: Query<
        (&mut Transform, &mut OrthographicProjection, &mut Camera),
        (With<Camera>, Without<BallId>),
    >,
) {
    for (ball_id, t) in ball_query.iter() {
        for (mut transform, mut _ortho, _c) in query.iter_mut() {
            // if let Some(ci) = ci{
            //   if ci.show_ui{
            if ball_id == &local_user_info.0.ball_id {
                transform.translation.x = t.translation.x;
                transform.translation.y = t.translation.y;
                if transform.translation.x > 3700.0 {
                    transform.translation.x = 3700.0;
                }
                if transform.translation.x < 160.0 {
                    transform.translation.x = 160.0;
                }
                if transform.translation.y > 3700.0 {
                    transform.translation.y = 3700.0;
                }
                if transform.translation.y < 160.0 {
                    transform.translation.y = 160.0;
                }
                let z = transform.translation.z;
                // Important! We need to restore the Z values when moving the camera around.
                // Bevy has a specific camera setup and this can mess with how our layers are shown.
                transform.translation.z = z;
            }
        }
    }
}
