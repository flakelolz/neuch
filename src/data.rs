use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Hitbox {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HitboxGroup {
    pub start: i32,
    pub duration: i32,
    pub hitboxes: Vec<Hitbox>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Action {
    pub name: String,
    pub duration: i32,
    pub looping: bool,
    pub pushbox: Vec<HitboxGroup>,
    pub hurtbox: Vec<HitboxGroup>,
    pub hitbox: Vec<HitboxGroup>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CharacterData {
    pub health: i32,
    pub pushbox: Hitbox,
    pub actions: Vec<Action>,
}

impl CharacterData {
    pub fn load(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => json::from_str(&contents).unwrap(),
            Err(_) => Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct GameData {
    pub characters: Vec<CharacterData>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            characters: vec![CharacterData::load("assets/data/data.json")],
        }
    }
}
