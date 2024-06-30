use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Hitbox {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HitboxGroup {
    pub frame: i32,
    pub duration: i32,
    pub boxes: Vec<Hitbox>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
    pub total: i32,
    pub looping: bool,
    pub pushbox: Vec<HitboxGroup>,
    pub hurtbox: Vec<HitboxGroup>,
    pub hitbox: Option<Vec<HitboxGroup>>,
    pub modifiers: Option<Modifiers>,
    pub timeline: Vec<Keyframe>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Modifiers {
    pub chainable: Option<ChainModifier>,
    pub potisions: Option<Vec<PositionModifier>>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct PositionModifier {
    pub on_frame: i32,
    pub value: IVec2,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct ChainModifier {
    pub on_frame: i32,
    pub st_lp: bool,
    pub st_lk: bool,
    pub cr_lp: bool,
    pub cr_lk: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct Keyframe {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CharacterData {
    pub name: String,
    pub health: i32,
    pub actions: Vec<Action>,
}

impl CharacterData {
    pub fn load(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(content) => content,
                Err(e) => panic!("{}", e),
            },
            Err(_) => Default::default(),
        }
    }
}

#[allow(unused)]
pub struct Character {
    pub name: String,
    pub data: CharacterData,
    pub action_map: HashMap<String, Action>,
}

impl Character {
    pub fn ken() -> Self {
        let data = CharacterData::load("assets/data/ken_data.json");
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
