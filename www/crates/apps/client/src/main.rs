use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy::utils::Duration;
use plugin_physics_rapier::PhysicsPlugin;
use plugin_protocol::ProtocolPlugin;
use shared::SharedPlugin;
#[bevy_main]
pub fn main() {
    let mut app = App::new();

    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        title: String::from("game"),
        canvas: Some(String::from("#game")),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin)
    .add_plugin(SharedPlugin)
    .add_plugin(ProtocolPlugin);
    app.run();
}
