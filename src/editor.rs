use std::io::Write;

use enum_iterator::Sequence;

use crate::prelude::*;

pub struct Editor {
    width: u32,
    height: u32,
    loaded: bool,
    index: usize,
    state: State,
    name: String,
    looping: bool,
    property: Property,
    hitbox: Hitbox,
    old_hitbox: Hitbox,
    hitboxes_length: usize,
    hurtbox: Hurtbox,
    old_hurtbox: Hurtbox,
    hurtboxes_length: usize,
    pushbox: Pushbox,
    old_pushbox: Pushbox,
    pushboxes_length: usize,
    proximity: ProximityBox,
    old_proximity: ProximityBox,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            width: 174,
            height: 170,
            loaded: false,
            index: 0,
            state: State::StMediumPunch,
            name: String::new(),
            looping: false,
            property: Property::Hitbox,
            hitbox: Hitbox::default(),
            old_hitbox: Hitbox::default(),
            hitboxes_length: 0,
            hurtbox: Hurtbox::default(),
            old_hurtbox: Hurtbox::default(),
            hurtboxes_length: 0,
            pushbox: Pushbox::default(),
            old_pushbox: Pushbox::default(),
            pushboxes_length: 0,
            proximity: ProximityBox::default(),
            old_proximity: ProximityBox::default(),
        }
    }
    pub fn show_editor(
        &mut self,
        world: &mut World,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        debug: &Debug,
    ) {
        if !debug.editor {
            return;
        }

        let x = 6;
        let x2 = 62;
        d.gui_group_box(rrect(2, 6, self.width, self.height), Some(c"Editor"));

        // NOTE: Action
        d.gui_label(rrect(x, 12, 40, 8), Some(c"Action"));
        d.gui_status_bar(
            rrect(x2, 12, 100, 8),
            Some(rstr!("{}", &self.state.name()).as_c_str()),
        );
        if d.gui_label_button(rrect(50, 12, 1, 8), Some(c"<-")) {
            if let Some(previous) = self.state.previous() {
                self.index = 0;
                self.loaded = false;
                self.state = previous;
            }
        }
        if d.gui_label_button(rrect(self.width - 10, 12, 1, 8), Some(c"->")) {
            if let Some(next) = self.state.next() {
                self.index = 0;
                self.loaded = false;
                self.state = next;
            }
        }

        // NOTE: Property
        d.gui_status_bar(
            rrect(x + 96, 24, 60, 8),
            Some(rstr!("{}", &self.property.name()).as_c_str()),
        );
        if d.gui_label_button(rrect(x + 84, 24, 1, 8), Some(c"<-")) {
            if let Some(previous) = self.property.previous() {
                self.loaded = false;
                self.index = 0;
                self.property = previous;
            }
        }
        if d.gui_label_button(rrect(self.width - 10, 24, 1, 8), Some(c"->")) {
            if let Some(next) = self.property.next() {
                self.loaded = false;
                self.index = 0;
                self.property = next;
            }
        }

        // NOTE: Index
        d.gui_label(rrect(x, 24, 40, 8), Some(c"Index"));
        d.gui_status_bar(
            rrect(x + 44, 24, 24, 8),
            Some(rstr!("{}", &self.index).as_c_str()),
        );
        if d.gui_label_button(rrect(x + 32, 24, 30, 8), Some(c"<-")) {
            self.loaded = false;
            if self.index > 0 {
                self.index -= 1;
            }
        }
        if d.gui_label_button(rrect(x + 70, 24, 1, 8), Some(c"->")) {
            self.loaded = false;
            let length = {
                match self.property {
                    Property::Hitbox => self.hitboxes_length,
                    Property::Hurtbox => self.hurtboxes_length,
                    Property::Pushbox => self.pushboxes_length,
                    _ => 1,
                }
            };

            if self.index < length - 1 {
                self.index += 1;
            }
        }

        if d.gui_button(rrect(x, self.height - 10, 60, 10), Some(c"Save")) {
            self.save();
        }

        for (_, (character, player)) in world.query_mut::<(&mut Character, &Player)>() {
            self.name.clone_from(&character.name);
            let map = &mut character.action_map;
            let action = map.get_mut(&self.state.name());
            if let Some(action) = action {
                if player == &Player::One {
                    match self.property {
                        // NOTE: VALUES
                        Property::Values => {
                            if !self.loaded {
                                self.looping = action.looping;
                                self.loaded = true;
                            }
                            let y = 36;
                            let z = 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Looping"));
                            d.gui_check_box(rrect(x2, y + z, 8, 8), None, &mut action.looping);
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.looping = self.looping;
                            }
                        }
                        // NOTE: HITBOX
                        Property::Hitbox => {
                            if action.hitboxes.is_none() {
                                return;
                            }
                            let hitboxes = action.hitboxes.as_mut().unwrap();
                            if !self.loaded {
                                self.hitbox = hitboxes[self.index];
                                self.old_hitbox = self.hitbox;
                                self.hitboxes_length = hitboxes.len();
                                self.loaded = true;
                            }
                            let y = 36;
                            let z = 12;
                            d.gui_label(rrect(x, y, 60, 8), Some(c"Startup"));
                            let startup = &mut (hitboxes[self.index].start_frame as i32);
                            d.gui_value_box(rrect(x2, y, 30, 8), None, startup, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y, 10, 8), Some(c"-")) {
                                self.hitbox.start_frame -= 1;
                                action.hitboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.hitbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                self.hitbox.start_frame += 1;
                                action.hitboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.hitbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.old_hitbox.start_frame;
                                self.hitbox.start_frame = self.old_hitbox.start_frame;
                            }

                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                            let dur = &mut (self.hitbox.duration as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.duration -= 1;
                                action.hitboxes.as_mut().unwrap()[self.index].duration =
                                    self.hitbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.duration += 1;
                                action.hitboxes.as_mut().unwrap()[self.index].duration =
                                    self.hitbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].duration =
                                    self.old_hitbox.duration;
                                self.hitbox.duration = self.old_hitbox.duration;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Hitstop"));
                            let hitstop = &mut (self.hitbox.properties.hitstop as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, hitstop, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.hitstop -= 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstop = self.hitbox.properties.hitstop;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.hitstop += 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstop = self.hitbox.properties.hitstop;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstop = self.old_hitbox.properties.hitstop;
                                self.hitbox.properties.hitstop = self.old_hitbox.properties.hitstop;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Hitstun"));
                            let hitstun = &mut (self.hitbox.properties.hitstun as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, hitstun, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.hitstun -= 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstun = self.hitbox.properties.hitstun;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.hitstun += 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstun = self.hitbox.properties.hitstun;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .hitstun = self.old_hitbox.properties.hitstun;
                                self.hitbox.properties.hitstun = self.old_hitbox.properties.hitstun;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Blockstun"));
                            let blckstn = &mut (self.hitbox.properties.blockstun as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, blckstn, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.blockstun -= 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .blockstun = self.hitbox.properties.blockstun;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.blockstun += 1;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .blockstun = self.hitbox.properties.blockstun;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .blockstun = self.old_hitbox.properties.blockstun;
                                self.hitbox.properties.blockstun =
                                    self.old_hitbox.properties.blockstun;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Knockback"));
                            let kb = &mut self.hitbox.properties.knockback;
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, kb, 1, 99900, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.knockback -= 100;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .knockback = self.hitbox.properties.knockback;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.knockback += 100;
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .knockback = self.hitbox.properties.knockback;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index]
                                    .properties
                                    .knockback = self.old_hitbox.properties.knockback;
                                self.hitbox.properties.knockback =
                                    self.old_hitbox.properties.knockback;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                            let top = &mut self.hitbox.value.top;
                            let top = &mut world_to_screen_num(*top);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.value.top -= 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.top =
                                    self.hitbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.top += 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.top =
                                    self.hitbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].value.top =
                                    self.old_hitbox.value.top;
                                self.hitbox.value.top = self.old_hitbox.value.top;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                            let bottom = &mut self.hitbox.value.bottom;
                            let bottom = &mut world_to_screen_num(*bottom);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, bottom, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.value.bottom -= 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.hitbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.bottom += 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.hitbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.old_hitbox.value.bottom;
                                self.hitbox.value.bottom = self.old_hitbox.value.bottom;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                            let left = &mut self.hitbox.value.left;
                            let left = &mut world_to_screen_num(*left);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, left, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.value.left -= 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.left =
                                    self.hitbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.left += 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.left =
                                    self.hitbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].value.left =
                                    self.old_hitbox.value.left;
                                self.hitbox.value.left = self.old_hitbox.value.left;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                            let right = &mut self.hitbox.value.right;
                            let right = &mut world_to_screen_num(*right);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, right, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.value.right -= 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.right =
                                    self.hitbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.right += 1000;
                                action.hitboxes.as_mut().unwrap()[self.index].value.right =
                                    self.hitbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[self.index].value.right =
                                    self.old_hitbox.value.right;
                                self.hitbox.value.right = self.old_hitbox.value.right;
                            }

                            if d.gui_button(
                                rrect(self.width - 50, self.height - 10, 40, 10),
                                Some(c"Add"),
                            ) {
                                action.hitboxes.as_mut().unwrap().push(self.hitbox);
                                self.hitboxes_length = action.hitboxes.as_ref().unwrap().len();
                                self.index = self.hitboxes_length - 1;
                                self.loaded = false;
                            }
                        }
                        // NOTE: HURTBOX
                        Property::Hurtbox => {
                            if action.hurtboxes.is_none() {
                                return;
                            }
                            if !self.loaded {
                                let hurtboxes = action.hurtboxes.as_ref().unwrap();
                                self.hurtbox = hurtboxes[self.index];
                                self.old_hurtbox = self.hurtbox;
                                self.hurtboxes_length = hurtboxes.len();
                                self.loaded = true;
                            }
                            let y = 36;
                            let z = 12;
                            d.gui_label(rrect(x, y, 60, 8), Some(c"Startup"));
                            let startup = &mut (self.hurtbox.start_frame as i32);
                            d.gui_value_box(rrect(x2, y, 30, 8), None, startup, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y, 10, 8), Some(c"-")) {
                                self.hurtbox.start_frame -= 1;
                                action.hurtboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.hurtbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                self.hurtbox.start_frame += 1;
                                action.hurtboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.hurtbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].start_frame =
                                    self.old_hurtbox.start_frame;
                                self.hurtbox.start_frame = self.old_hurtbox.start_frame;
                            }

                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                            let dur = &mut (self.hurtbox.duration as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.duration -= 1;
                                action.hurtboxes.as_mut().unwrap()[self.index].duration =
                                    self.hurtbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.duration += 1;
                                action.hurtboxes.as_mut().unwrap()[self.index].duration =
                                    self.hurtbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].duration =
                                    self.old_hurtbox.duration;
                                self.hurtbox.duration = self.old_hurtbox.duration;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                            let top = &mut self.hurtbox.value.top;
                            let top = &mut world_to_screen_num(*top);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.top -= 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.top =
                                    self.hurtbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.top += 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.top =
                                    self.hurtbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].value.top =
                                    self.old_hurtbox.value.top;
                                self.hurtbox.value.top = self.old_hurtbox.value.top;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                            let bottom = &mut self.hurtbox.value.bottom;
                            let bottom = &mut world_to_screen_num(*bottom);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, bottom, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.bottom -= 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.hurtbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.bottom += 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.hurtbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].value.bottom =
                                    self.old_hurtbox.value.bottom;
                                self.hurtbox.value.bottom = self.old_hurtbox.value.bottom;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                            let left = &mut self.hurtbox.value.left;
                            let left = &mut world_to_screen_num(*left);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, left, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.left -= 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.left =
                                    self.hurtbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.left += 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.left =
                                    self.hurtbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].value.left =
                                    self.old_hurtbox.value.left;
                                self.hurtbox.value.left = self.old_hurtbox.value.left;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                            let right = &mut self.hurtbox.value.right;
                            let right = &mut world_to_screen_num(*right);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, right, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.right -= 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.right =
                                    self.hurtbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.right += 1000;
                                action.hurtboxes.as_mut().unwrap()[self.index].value.right =
                                    self.hurtbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[self.index].value.right =
                                    self.old_hurtbox.value.right;
                                self.hurtbox.value.right = self.old_hurtbox.value.right;
                            }

                            if d.gui_button(
                                rrect(self.width - 50, self.height - 10, 40, 10),
                                Some(c"Add"),
                            ) {
                                action.hurtboxes.as_mut().unwrap().push(self.hurtbox);
                                self.hurtboxes_length = action.hurtboxes.as_ref().unwrap().len();
                                self.index = self.hurtboxes_length - 1;
                                self.loaded = false;
                            }
                        }
                        // NOTE: PUSHBOX
                        Property::Pushbox => {
                            if let Some(pushboxes) = &mut action.pushboxes {
                                if !self.loaded {
                                    self.pushbox = pushboxes[self.index];
                                    self.old_pushbox = self.pushbox;
                                    self.loaded = true;
                                }
                                let y = 36;
                                let z = 12;
                                d.gui_label(rrect(x, y, 60, 8), Some(c"Startup"));
                                let startup = &mut (self.pushbox.start_frame as i32);
                                d.gui_value_box(rrect(x2, y, 30, 8), None, startup, 1, 100, false);
                                if d.gui_label_button(rrect(x2 + 34, y, 10, 8), Some(c"-")) {
                                    self.pushbox.start_frame -= 1;
                                    action.pushboxes.as_mut().unwrap()[self.index].start_frame =
                                        self.pushbox.start_frame;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                    self.pushbox.start_frame += 1;
                                    action.pushboxes.as_mut().unwrap()[self.index].start_frame =
                                        self.pushbox.start_frame;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                    action.pushboxes.as_mut().unwrap()[self.index].start_frame =
                                        self.old_pushbox.start_frame;
                                    self.pushbox.start_frame = self.old_pushbox.start_frame;
                                }

                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                                let dur = &mut (self.pushbox.duration as i32);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.duration -= 1;
                                    action.pushboxes.as_mut().unwrap()[self.index].duration =
                                        self.pushbox.duration;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.duration += 1;
                                    action.pushboxes.as_mut().unwrap()[self.index].duration =
                                        self.pushbox.duration;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.pushboxes.as_mut().unwrap()[self.index].duration =
                                        self.old_pushbox.duration;
                                    self.pushbox.duration = self.old_pushbox.duration;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                                let top = &mut self.pushbox.value.top;
                                let top = &mut world_to_screen_num(*top);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.top -= 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.top =
                                        self.pushbox.value.top;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.top += 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.top =
                                        self.pushbox.value.top;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.pushboxes.as_mut().unwrap()[self.index].value.top =
                                        self.old_pushbox.value.top;
                                    self.pushbox.value.top = self.old_pushbox.value.top;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                                let bottom = &mut self.pushbox.value.bottom;
                                let btm = &mut world_to_screen_num(*bottom);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, btm, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.bottom -= 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.bottom =
                                        self.pushbox.value.bottom;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.bottom += 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.bottom =
                                        self.pushbox.value.bottom;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.pushboxes.as_mut().unwrap()[self.index].value.bottom =
                                        self.old_pushbox.value.bottom;
                                    self.pushbox.value.bottom = self.old_pushbox.value.bottom;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                                let left = &mut self.pushbox.value.left;
                                let lft = &mut world_to_screen_num(*left);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, lft, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.left -= 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.left =
                                        self.pushbox.value.left;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.left += 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.left =
                                        self.pushbox.value.left;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.pushboxes.as_mut().unwrap()[self.index].value.left =
                                        self.old_pushbox.value.left;
                                    self.pushbox.value.left = self.old_pushbox.value.left;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                                let right = &mut self.pushbox.value.right;
                                let rgt = &mut world_to_screen_num(*right);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, rgt, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.right -= 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.right =
                                        self.pushbox.value.right;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.right += 1000;
                                    action.pushboxes.as_mut().unwrap()[self.index].value.right =
                                        self.pushbox.value.right;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.pushboxes.as_mut().unwrap()[self.index].value.right =
                                        self.old_pushbox.value.right;
                                    self.pushbox.value.right = self.old_pushbox.value.right;
                                }
                            }
                        }
                        // NOTE: PROXIMITY
                        Property::Proximity => {
                            if action.modifiers.is_none() {
                                return;
                            }
                            if action.modifiers.as_ref().unwrap().proximity.is_none() {
                                return;
                            }
                            let proximity = action
                                .modifiers
                                .as_mut()
                                .unwrap()
                                .proximity
                                .as_mut()
                                .unwrap();
                            if !self.loaded {
                                self.proximity = *proximity;
                                self.old_proximity = self.proximity;
                                self.loaded = true;
                            }
                            let y = 36;
                            let z = 12;
                            d.gui_label(rrect(x, y, 60, 8), Some(c"Startup"));
                            let startup = &mut (self.proximity.start_frame as i32);
                            d.gui_value_box(rrect(x2, y, 30, 8), None, startup, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y, 10, 8), Some(c"-")) {
                                self.proximity.start_frame -= 1;
                                proximity.start_frame = self.proximity.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                self.proximity.start_frame += 1;
                                proximity.start_frame = self.proximity.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                self.proximity.start_frame = self.old_proximity.start_frame;
                                proximity.start_frame = self.old_proximity.start_frame;
                            }

                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                            let dur = &mut (self.proximity.duration as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.proximity.duration -= 1;
                                proximity.duration = self.proximity.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.proximity.duration += 1;
                                proximity.duration = self.proximity.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                self.proximity.duration = self.old_proximity.duration;
                                proximity.duration = self.old_proximity.duration;
                            }
                            // HERE
                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                            let top = &mut self.proximity.value.top;
                            let top = &mut world_to_screen_num(*top);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.proximity.value.top -= 1000;
                                proximity.value.top = self.proximity.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.proximity.value.top += 1000;
                                proximity.value.top = self.proximity.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                self.proximity.value.top = self.old_proximity.value.top;
                                proximity.value.top = self.old_proximity.value.top;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                            let bottom = &mut self.proximity.value.bottom;
                            let bottom = &mut world_to_screen_num(*bottom);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, bottom, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.proximity.value.bottom -= 1000;
                                proximity.value.bottom = self.proximity.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.proximity.value.bottom += 1000;
                                proximity.value.bottom = self.proximity.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                self.proximity.value.bottom = self.old_proximity.value.bottom;
                                proximity.value.bottom = self.old_proximity.value.bottom;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                            let left = &mut self.proximity.value.left;
                            let left = &mut world_to_screen_num(*left);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, left, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.proximity.value.left -= 1000;
                                proximity.value.left = self.proximity.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.proximity.value.left += 1000;
                                proximity.value.left = self.proximity.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                self.proximity.value.left = self.old_proximity.value.left;
                                proximity.value.left = self.old_proximity.value.left;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                            let right = &mut self.proximity.value.right;
                            let right = &mut world_to_screen_num(*right);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, right, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.proximity.value.right -= 1000;
                                proximity.value.right = self.proximity.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.proximity.value.right += 1000;
                                proximity.value.right = self.proximity.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                self.proximity.value.right = self.old_proximity.value.right;
                                proximity.value.right = self.old_proximity.value.right;
                            }
                        }
                    }
                }
            }
        }
        if d.gui_button(rrect(x + 70, self.height - 10, 30, 10), Some(c"2P")) {
            let mut players = world
                .query_mut::<&mut Character>()
                .into_iter()
                .collect::<Vec<_>>();
            let split = &mut players.split_at_mut(1);
            let (p1, p2) = split;
            if let Some((_, a_character)) = p1.get_mut(0) {
                if let Some((_, b_character)) = p2.get_mut(0) {
                    b_character.action_map.clone_from(&a_character.action_map);
                }
            }
        }
    }

    fn save(&mut self) {
        let name = format!("assets/data/{}_data.json", self.name);
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(name)
            .unwrap();

        let mut data: CharacterData = serde_json::from_reader(file).unwrap();

        for action in &mut data.actions {
            if action.name == self.state.name() {
                match self.property {
                    Property::Values => {
                        action.looping = self.looping;
                    }
                    Property::Hitbox => {
                        if let Some(hitboxes) = &mut action.hitboxes {
                            hitboxes[self.index] = self.hitbox;
                        }
                    }
                    Property::Hurtbox => {
                        if let Some(hurtboxes) = &mut action.hurtboxes {
                            hurtboxes[self.index] = self.hurtbox;
                        }
                    }
                    Property::Pushbox => {
                        if let Some(pushboxes) = &mut action.pushboxes {
                            pushboxes[self.index] = self.pushbox;
                        }
                    }
                    Property::Proximity => {
                        if let Some(modifiers) = &mut action.modifiers {
                            if let Some(proximity) = &mut modifiers.proximity {
                                *proximity = self.proximity;
                            }
                        }
                    }
                }

                break;
            }
        }

        let output_name = format!("assets/data/{}_data.json", self.name);
        let mut file = std::fs::File::create(output_name).unwrap();
        file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
            .unwrap();
    }
}

#[derive(Sequence)]
enum State {
    StIdle,
    StWalkForward,
    StWalkBackward,
    StLightPunch,
    StMediumPunch,
    StHeavyPunch,
    StLightKick,
    StMediumKick,
    StHeavyKick,
    CrStart,
    CrIdle,
    CrEnd,
    CrLightPunch,
    CrMediumPunch,
    CrHeavyPunch,
    CrLightKick,
    CrMediumKick,
    CrHeavyKick,
    JmpStart,
    JmpNeutral,
    JmpForward,
    JmpBackward,
    JmpLightPunch,
    JmpMediumPunch,
    JmpHeavyPunch,
    JmpLightKick,
    JmpMediumKick,
    JmpHeavyKick,
}

impl State {
    pub fn name(&self) -> String {
        match self {
            State::StIdle => "St Idle".to_string(),
            State::StWalkForward => "St WalkForward".to_string(),
            State::StWalkBackward => "St WalkBackward".to_string(),
            State::StLightPunch => "St LightPunch".to_string(),
            State::StMediumPunch => "St MediumPunch".to_string(),
            State::StHeavyPunch => "St HeavyPunch".to_string(),
            State::StLightKick => "St LightKick".to_string(),
            State::StMediumKick => "St MediumKick".to_string(),
            State::StHeavyKick => "St HeavyKick".to_string(),
            State::CrStart => "Cr Start".to_string(),
            State::CrIdle => "Cr Idle".to_string(),
            State::CrEnd => "Cr End".to_string(),
            State::CrLightPunch => "Cr LightPunch".to_string(),
            State::CrMediumPunch => "Cr MediumPunch".to_string(),
            State::CrHeavyPunch => "Cr HeavyPunch".to_string(),
            State::CrLightKick => "Cr LightKick".to_string(),
            State::CrMediumKick => "Cr MediumKick".to_string(),
            State::CrHeavyKick => "Cr HeavyKick".to_string(),
            State::JmpStart => "Jmp Start".to_string(),
            State::JmpNeutral => "Jmp Neutral".to_string(),
            State::JmpForward => "Jmp Forward".to_string(),
            State::JmpBackward => "Jmp Backward".to_string(),
            State::JmpLightPunch => "Jmp LightPunch".to_string(),
            State::JmpMediumPunch => "Jmp MediumPunch".to_string(),
            State::JmpHeavyPunch => "Jmp HeavyPunch".to_string(),
            State::JmpLightKick => "Jmp LightKick".to_string(),
            State::JmpMediumKick => "Jmp MediumKick".to_string(),
            State::JmpHeavyKick => "Jmp HeavyKick".to_string(),
        }
    }
}

#[derive(Sequence)]
enum Property {
    Values,
    Hitbox,
    Hurtbox,
    Pushbox,
    Proximity,
}

impl Property {
    pub fn name(&self) -> String {
        match self {
            Property::Values => "Values".to_string(),
            Property::Hitbox => "Hitbox".to_string(),
            Property::Hurtbox => "Hurtbox".to_string(),
            Property::Pushbox => "Pushbox".to_string(),
            Property::Proximity => "Proximity".to_string(),
        }
    }
}
