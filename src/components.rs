use std::fmt;

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
    Equipped { by: Entity },
    Carried { by: Entity }
}

pub struct Name {
    pub name: String
}
impl Name {
    pub fn new() -> Self{
        Self { 
            name: RNG::try_from(&Language::Elven)
            .expect("random name could not be generated")
            .generate_name(),
        }
    }
}

#[derive(Clone)]
pub struct Person {
    pub coolness: i32
}

impl Person {
    pub fn new() -> Self{
        Self { 
            coolness: rand::thread_rng().gen_range(1..11), 
        }
    }
}

struct PriorityQueue {

}

pub struct Hunger {
    pub current_hunger: u32,
    pub  max_hunger: u32
}

impl Hunger {
    pub fn new(hunger: u32) -> Self{
        Self {
            current_hunger: hunger,
            max_hunger: hunger
        }
    }
    pub fn random() -> Self{
        let h = rand::thread_rng().gen_range(1..11);
        Self {
            current_hunger: h,
            max_hunger: h
        }
    }
}


#[derive(Clone, Debug)]
pub enum Targets {
    Single { target : Entity },
    TargetList { targets: Vec<Entity> },
    Tile { tile_idx : i32 },
    Tiles { tiles: Vec<i32> }
}


enum Action {
    Destroy,
    Heal,
    Aquire,
    Enjoy,
    Mourn,
}


#[derive(Debug, Clone)]
pub struct ProvidesHealing {
    pub heal_amount : u32
}

#[derive(Debug, Clone)]
pub struct ProvidesDamage {
    pub damage_amount : u32
}

pub struct Item {
    pub weight: u32,
    pub value: u32,
    pub level: u32
}

pub struct Living {
    pub alive: bool
}
impl Living {
    pub fn new()-> Self{
        Self{ alive: true}
    }
}

pub struct Health {
    pub current: i32,
    pub max: i32
}
impl Health {
    pub fn new(health_bonus: i32)-> Self{
        Self{
            current: 8 + health_bonus,
            max: 8 + health_bonus
        }
    }
}


pub struct BaseStats{
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub charisma: i32,
    pub intelligence: i32,
    pub wisdom: i32,
}
impl BaseStats {
    pub fn random()-> Self{
        Self{
            strength: rand::thread_rng().gen_range(-8..8),
            dexterity: rand::thread_rng().gen_range(-8..8),
            constitution: rand::thread_rng().gen_range(-8..8),
            charisma: rand::thread_rng().gen_range(-8..8),
            intelligence: rand::thread_rng().gen_range(-8..8),
            wisdom: rand::thread_rng().gen_range(-8..8), 
        }
    }

    // pub fn make_warrior() -> Self{
    //     Self { strength: (), dexterity: (), constitution: (), charisma: (), intelligence: (), wisdom: () }
    // }
}
impl fmt::Display for BaseStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BaseStats {{\n  Strength: {}\n  Dexterity: {}\n  Constitution: {}\n  Charisma: {}\n  Intelligence: {}\n  Wisdom: {}\n}}",
            self.strength,
            self.dexterity,
            self.constitution,
            self.charisma,
            self.intelligence,
            self.wisdom
        )
    }
}


pub type People<'a> = (&'a Name, &'a Person, &'a Hunger, &'a BaseStats, &'a Health);