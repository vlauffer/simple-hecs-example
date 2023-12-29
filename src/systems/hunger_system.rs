use crate::prelude::*;

pub fn hunger_system(world: &World, cmd: &mut CommandBuffer){
    for (e, h) in world.query::<&Hunger>().iter(){
        
    }
}