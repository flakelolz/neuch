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
    pub timeline: Vec<Timeline>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CharacterData {
    pub name: String,
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

pub struct Character {
    pub name: String,
    pub data: CharacterData,
    pub action_map: HashMap<String, Action>,
}

impl Character {
    pub fn ken() -> Self {
        let data = CharacterData::load("assets/data/data.json");
        let action_map = generate_action_map(&data);
        Self {
            name: "Ken".to_string(),
            data,
            action_map,
        }
    }
}

pub fn generate_action_map(character: &CharacterData) -> HashMap<String, Action> {
    let mut hashmap = HashMap::new();

    for action in &character.actions {
        hashmap.insert(action.name.clone(), action.clone());
    }

    hashmap
}

pub fn find_action<'a>(character: &'a Character, action_name: &'a String) -> Option<&'a Action> {
    character.action_map.get(action_name)
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Timeline {
    index: i32,
    start: i32,
    duration: i32,
}
