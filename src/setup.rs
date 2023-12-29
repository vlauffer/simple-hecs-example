use crate::prelude::*;

pub fn setup_world(world: &mut World){
    setup_people(world);
    setup_resources(world);

}

pub fn setup_people(world: &mut World){

    world.spawn_batch((3..7).map(|_| {
        let random_base = BaseStats::random();
        (
            Person::new(), 
            Name::new(), 
            Hunger::random(), 
            Health::new(random_base.constitution),
            random_base
        )
    }));
}

pub enum Effect {
    Healing {e: ProvidesHealing},
    Damage {e: ProvidesDamage},
}

// ew
pub fn setup_resources(world: &mut World){
    let mut builder = EntityBuilder::new();
    let mut rng = thread_rng();
    
    for i in 3..7 {
        let level = rng.gen_range(1..4) as u32;
        let item_base = Item { weight: rng.gen_range(1..4), value: level.pow(3), level };
        let healing_item = ProvidesHealing {heal_amount: level.pow(2)};
        let spawn_type = SpawnType::AtPosition { x: rng.gen_range(0..10), y: rng.gen_range(0..10) };
        let item_name = Name{name: format!("Potion of healing, level {}", level).to_owned()};
        world.spawn((item_base, healing_item, spawn_type, item_name));
    }

    for i in 3..7 {
        let level = rng.gen_range(1..4);
        let item_base = Item { weight: rng.gen_range(1..4), value: 3^level, level };
        let damage_item = ProvidesDamage {damage_amount: 2^level};
        let spawn_type = SpawnType::AtPosition { x: 0, y: 0 };
        let item_name = Name{name: format!("Potion of damage, level {}", level).to_owned()};
        world.spawn((item_base, damage_item, spawn_type, item_name));
    }
}

pub fn setup_goals(world: &mut World){
    
}