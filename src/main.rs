use hecs::*;
use rnglib::{RNG, Language};
use rand::{Rng};

#[derive(Clone)]
struct Person {
    name: String,
    coolness: i32
}

impl Person {
    fn new() -> Self{
        Self { 
            name: RNG::try_from(&Language::Elven)
            .expect("random name could not be generated")
            .generate_name(),
            coolness: rand::thread_rng().gen_range(1..11), 
        }
    }
}

struct WantsToTalkTo{
    entities: Entity
}

fn wants_to_talk_to_system(world: &World, cmd: &mut CommandBuffer) {
    let mut ent_holder = world.query::<(&Person)>();
    let mut coolness_vec: Vec<(Entity, &Person)> = ent_holder.iter()
        .map(|(e,p)| (e, p))
        .collect();
    coolness_vec.sort_by(|a,b| a.1.coolness.cmp(&b.1.coolness));
    for (e, p) in world.query::<(&Person)>().iter(){
        let search_key = p.coolness;
        let insert_index: usize = match coolness_vec.binary_search_by(|(a, s)| s.coolness.cmp(&search_key)){
            Ok(index) => {index},
            Err(index) =>  {index}
        };
        if insert_index < coolness_vec.len()-1 && e==coolness_vec[insert_index+1].0{
            cmd.insert_one(e, WantsToTalkTo{entities: coolness_vec[insert_index+1].0});
        }
    }
}

fn main(){
    let mut world = World::default();
    let mut cmd = CommandBuffer::new();
    world.spawn_batch((3..7).map(|_| (Person::new(),) ));
    wants_to_talk_to_system(&world, &mut cmd);
    cmd.run_on(&mut world);
    for (e) in world.iter(){
        if let Some(person) = e.get::<&Person>(){
            println!("{} is here. Coolness: {}", person.name, person.coolness);
        }
        if let Some(entity_to_talk_to) = e.get::<&WantsToTalkTo>(){
            let e = entity_to_talk_to.entities;
            if let Ok(retrieved_entity) = world.entity(e) {
                let target = &retrieved_entity.get::<&Person>().expect("Could not unwrap person").name;
                println!("They want to talk to {}", target);
            }
        } else{
            println!("They are too cool for everyone here");
        }
    }
}

fn print_world(world: &World){
    let mut query = world.query::<(&Person)>();
    for (e,p) in query.iter(){
        match p.coolness > 7 {
            true => println!("{} is so cool! Cool level {}", p.name, p.coolness),
            false => println!("{} is lame :( Cool level {}", p.name, p.coolness),
        }
	}
}

