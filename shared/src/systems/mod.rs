
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
pub mod physics;
pub use physics::*;
pub mod entity_to_remove;
pub use entity_to_remove::*;
use crate::*;