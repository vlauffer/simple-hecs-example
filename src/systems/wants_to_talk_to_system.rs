use crate::prelude::*;

pub fn wants_to_talk_to_system(world: &World, cmd: &mut CommandBuffer) {
  let mut ent_holder = world.query::<&Person>();
  let mut coolness_vec: Vec<(Entity, &Person)> = ent_holder.iter()
      .map(|(e,p)| (e, p))
      .collect();
  coolness_vec.sort_by(|a,b| a.1.coolness.cmp(&b.1.coolness));
  for (e, p) in world.query::<&Person>().iter(){
      let search_key = p.coolness;
      let insert_index: usize = match coolness_vec.binary_search_by(|(a, s)| s.coolness.cmp(&search_key)){
          Ok(index) => {index},
          Err(index) =>  {index}
      };
      if insert_index < coolness_vec.len()-1 && e!=coolness_vec[insert_index+1].0{
        println!("adding");
          // cmd.insert_one(e, WantsToTalkTo{entities: coolness_vec[insert_index+1].0});
      }
  }
}