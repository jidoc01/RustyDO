use std::cmp::Ordering;

use super::*;

pub fn create(world: &mut World, info: GameInfo) -> &mut Entity {
    let id = world.push();
    let entity = world.get_mut(&id).unwrap();
    entity.push(EntityKind::Game);
    entity.push(info);
    entity
}
