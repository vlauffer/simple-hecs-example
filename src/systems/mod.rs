use crate::prelude::*;
pub mod goals_system;
pub mod wants_to_talk_to_system;
pub mod hunger_system;
pub mod death_system;

pub fn run_systems(world: &World, cmd: &mut CommandBuffer) {
    wants_to_talk_to_system::wants_to_talk_to_system(world, cmd);
    goals_system::goals_system(world, cmd);
}