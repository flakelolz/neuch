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
    pub frame: i32,
    pub duration: i32,
    pub hitboxes: Vec<Hitbox>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Action {
    pub name: String,
    pub total: i32,
    pub looping: bool,
    pub pushbox: Vec<HitboxGroup>,
    pub hurtbox: Vec<HitboxGroup>,
    pub hitbox: Vec<HitboxGroup>,
    pub timeline: Vec<Keyframe>,
}

#[derive(Debug, Clone, Copy, Deserialize, Default)]
pub struct Keyframe {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CharacterData {
    pub name: String,
    pub health: i32,
    pub actions: Vec<Action>,
}

impl CharacterData {
    pub fn load(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => match json::from_str(&contents) {
                Ok(content) => content,
                Err(e) => panic!("{}", e),
            },
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
            name: data.name.clone(),
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
