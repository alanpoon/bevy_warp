use bevy::prelude::*;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
mod bundle;
mod systems;
mod plugin;
pub use bundle::*;
pub mod to_despawn;
pub mod update;
pub use plugin::SharedPlugin;
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32);
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum ServerMessage {
    GameState{ball_bundles:Vec<BallBundle>},
    TargetVelocity{ball_id:BallId,target_velocity:Vec2},
    Welcome{ball_bundle:BallBundle},
}
#[derive(Component,Serialize, Deserialize, Default, Clone,Debug, PartialEq, Hash, Eq)]
pub struct BallLabel(pub String,pub String); //Label, Flag
#[derive(Serialize, Deserialize, Clone,Debug)]
pub enum ClientMessage {
    TargetVelocity{game_id:String,ball_id:BallId,target_velocity:Vec2},
    Welcome{game_id:String,ball_id:BallId,ball_label:BallLabel},
    SubWelcome,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,Default)]
pub struct UserInfo{
  pub ball_id:BallId,
}
#[derive(Component,Default,Debug)]
pub struct LocalUserInfo(pub UserInfo);

#[derive(Default,Debug,Clone)]
pub struct EntityToRemove{
  pub entities: HashSet<Entity>
}
