use bevy::prelude::*;
use crate::*;
use crate::systems::entity_to_remove;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum MyLabel {
    Despawn,
}
pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
          .init_resource::<EntityToRemove>()
          .add_system_to_stage(CoreStage::Last,entity_to_remove::remove_entity_system.label(MyLabel::Despawn))
          
          ;
           
    }
  }