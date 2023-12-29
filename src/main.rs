mod systems;
mod components;
mod setup;
mod prelude{
    pub use rnglib::{RNG, Language};
    pub use hecs::*;
    pub use rand::{Rng, thread_rng};
    pub use crate::systems::*;
    pub use crate::components::*;
    pub use crate::setup::*;
}
use prelude::*;


fn main(){

    let mut world = World::default();
    let mut cmd = CommandBuffer::new();
    setup_world(&mut world);
    print_world(&world);

    // run_systems(&world, &mut cmd);
    // cmd.run_on(&mut world);

    // for (e) in world.iter(){
    //     if let Some(person) = e.get::<&Person>(){
    //         println!("{} is here. Coolness: {}", person.name, person.coolness);
    //     }
    //     if let Some(entity_to_talk_to) = e.get::<&WantsToTalkTo>(){
    //         let e = entity_to_talk_to.entities;
    //         if let Ok(retrieved_entity) = world.entity(e) {
    //             let target = &retrieved_entity.get::<&Person>().expect("Could not unwrap person").name;
    //             println!("They want to talk to {}", target);
    //         }
    //     } else{
    //         println!("They are too cool for everyone here");
    //     }
    // }
}

fn print_world(world: &World){
    for (e, (n, p , hu, bs, he)) in world.query::<(People)>().iter(){
        println!("Entity name: {}", n.name);
        println!("Hunger: {}", hu.current_hunger);
        println!("Current health: {}", he.current);
        println!("{}", bs);
    }
}

