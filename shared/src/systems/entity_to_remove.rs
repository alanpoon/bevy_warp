use crate::*;

pub fn remove_entity_system(
  mut cmd: Commands,
  mut res: ResMut<EntityToRemove>,
) {
  for e in (*res).entities.drain(){
    cmd.entity(e).despawn_recursive();
  }
}