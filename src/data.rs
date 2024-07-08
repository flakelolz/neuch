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
    pub on_frame: u32,
    pub duration: u32,
    pub boxes: Vec<Hitbox>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
    pub total: u32,
    pub looping: bool,
    pub pushbox: Vec<HitboxGroup>,
    pub hurtbox: Vec<HitboxGroup>,
    pub hitbox: Option<Vec<HitboxGroup>>,
    pub modifiers: Option<Modifiers>,
    pub timeline: Vec<Keyframe>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Modifiers {
    pub cancels: Option<Vec<CancelModifier>>,
    pub potisions: Option<Vec<PositionModifier>>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct PositionModifier {
    pub on_frame: u32,
    pub value: IVec2,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelModifier {
    pub on_frame: u32,
    pub states: Vec<States>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct Keyframe {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub duration: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CharacterData {
    pub name: String,
    pub health: i32,
    pub forward_walk: i32,
    pub backward_walk: i32,
    pub jump_velocity: i32,
    pub jump_deceleration: i32,
    pub jump_forward: i32,
    pub jump_backward: i32,
    pub origin: Vec2,
    pub actions: Vec<Action>,
}

impl CharacterData {
    pub fn load(path: &str) -> Self {
        match get_file(path) {
            Some(contents) => match serde_json::from_slice(contents) {
                Ok(content) => content,
                Err(e) => panic!("{}", e),
            },
            None => Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CharacterInfo {
    pub health: i32,
    pub walk_forward: i32,
    pub walk_backward: i32,
    pub jump_velocity: i32,
    pub jump_deceleration: i32,
    pub jump_forward: i32,
    pub jump_backward: i32,
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct Character {
    pub name: String,
    pub info: CharacterInfo,
    pub data: CharacterData,
    pub action_map: HashMap<String, Action>,
}

impl Character {
    pub fn ken() -> Self {
        let data = CharacterData::load("data/Ken_data.json");
        let info = CharacterInfo {
            health: data.health,
            walk_forward: data.forward_walk,
            walk_backward: data.backward_walk,
            jump_velocity: data.jump_velocity,
            jump_deceleration: data.jump_deceleration,
            jump_forward: data.jump_forward,
            jump_backward: data.jump_backward,
        };
        let action_map = generate_action_map(&data);
        Self {
            name: data.name.clone(),
            info,
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
