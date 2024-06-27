use std::io::Write;

use aseprite::SpritesheetData;

use crate::prelude::{CharacterData, Keyframe};

// PERF: This can be done more efficiently
pub fn parse_animation_name(filename: &str) -> (String, usize) {
    let start = filename.find('#').unwrap();
    let end = filename.find('.').unwrap();
    let name = String::from(&filename[start + 1..end]);

    let mut split = name.split_whitespace();
    let state = split.next().unwrap();
    let action = split.next().unwrap();
    let index = split.next().unwrap();

    let mut action_name = String::new();
    action_name.push_str(state);
    action_name.push(' ');
    action_name.push_str(action);

    (action_name, index.parse().unwrap())
}

// PERF: This aswell probably
pub fn update_animation_data(name: &str) {
    let data_name = format!("assets/data/{}_data.json", name);
    let anim_name = format!("assets/sprites/{}_anim.json", name);
    let data_file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(data_name)
        .unwrap();

    let anim_file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(anim_name)
        .unwrap();

    let mut data: CharacterData = serde_json::from_reader(data_file).unwrap();
    let anim: SpritesheetData = serde_json::from_reader(anim_file).unwrap();

    let mut action_total = 0;

    for action in data.actions.iter_mut() {
        action.timeline.clear();

        for frame in anim.frames.iter() {
            let (name, _) = parse_animation_name(&frame.filename);
            if action.name != name {
                continue;
            }

            action.timeline.push(Keyframe {
                x: frame.frame.x as i32,
                y: frame.frame.y as i32,
                w: frame.frame.w as i32,
                h: frame.frame.h as i32,
                duration: frame.duration as i32 / 16,
            });

            action_total += frame.duration as i32 / 16;
        }

        action.total = action_total;
        println!("{} -> {}: {}", data.name, action.name, action_total);

        action_total = 0;
    }

    let output_name = format!("assets/data/{}_data.json", name);
    let mut file = std::fs::File::create(output_name).unwrap();
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
        .unwrap();
}

pub fn update_all_data() {
    let data_files = std::fs::read_dir("assets/data").unwrap();

    for file in data_files {
        let file_name = file.unwrap().file_name();
        let file_name = file_name.to_str().unwrap();
        let char_name = file_name.split('_').next().unwrap();
        update_animation_data(char_name);
    }
}
