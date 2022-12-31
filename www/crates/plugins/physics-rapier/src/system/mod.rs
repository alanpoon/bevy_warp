use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle,Mesh2dHandle};
use shared::*;
pub fn add_shape(mut cmd: Commands,ball_query:Query<(Entity,&BallId,&BallLabel,&Transform),Without<Mesh2dHandle>>,
mut materials: ResMut<Assets<ColorMaterial>>,
mut meshes: ResMut<Assets<Mesh>>){
    // let font_handle = asset_server
    //   .load("fonts/FiraSans-Bold.ttf");
    for (entity,ball,ball_label,transform) in ball_query.iter(){
        cmd.entity(entity).insert_bundle(MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::new(50.).into()).into(),
          material: materials.add(ColorMaterial::from(Color::PURPLE)),
          transform: transform.clone(),
          ..default()
         })
          // .with_children(|parent| {
          //     let text_style = TextStyle {
          //       font:font_handle.clone(),
          //       font_size: 30.0,
          //       color: Color::BLACK,
          //     };
          //     let _text_alignment = TextAlignment {
          //       vertical: VerticalAlign::Center,
          //       horizontal: HorizontalAlign::Center,
          //     };
              
          //     parent.spawn_bundle(Text2dBundle {
          //       text: Text::from_section(&ball_label.0,text_style.clone()),
          //       transform: Transform::from_xyz(0.0,-100.0,3.0),
          //       ..Default::default()
          //     });
          //  })
           ;
    }
}