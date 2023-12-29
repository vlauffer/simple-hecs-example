use crate::prelude::*;

pub fn death_system(world: &World, cmd: &mut CommandBuffer){
    for (e, h) in world.query::<&Hunger>().iter(){
        
    }
}