use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

}