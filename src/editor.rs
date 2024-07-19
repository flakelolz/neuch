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
        }
    }
    pub fn show_editor(
        &mut self,
        world: &mut World,
        d: &mut RaylibTextureMode<RaylibDrawHandle>,
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
                self.loaded = false;
                self.state = previous;
            }
        }
        if d.gui_label_button(rrect(self.width - 10, 12, 1, 8), Some(c"->")) {
            if let Some(next) = self.state.next() {
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
            if player == &Player::One {
                self.name.clone_from(&character.name);
                let map = &mut character.action_map;
                let action = map.get_mut(&self.state.name());
                if let Some(action) = action {
                    match self.property {
                        // NOTE: HITBOX
                        Property::Hitbox => {
                            if action.hitboxes.is_none() {
                                return;
                            }
                            if !self.loaded {
                                let hitboxes = action.hitboxes.as_ref().unwrap();
                                self.hitbox = hitboxes[self.index];
                                self.old_hitbox = self.hitbox;
                                self.hitboxes_length = hitboxes.len();
                                self.loaded = true;
                            }
                            let y = 36;
                            let z = 12;
                            d.gui_label(rrect(x, y, 60, 8), Some(c"Startup"));
                            let startup = &mut (self.hitbox.start_frame as i32);
                            d.gui_value_box(rrect(x2, y, 30, 8), None, startup, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y, 10, 8), Some(c"-")) {
                                self.hitbox.start_frame -= 1;
                                action.hitboxes.as_mut().unwrap()[0].start_frame =
                                    self.hitbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                self.hitbox.start_frame += 1;
                                action.hitboxes.as_mut().unwrap()[0].start_frame =
                                    self.hitbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].start_frame =
                                    self.old_hitbox.start_frame;
                                self.hitbox.start_frame = self.old_hitbox.start_frame;
                            }

                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                            let dur = &mut (self.hitbox.duration as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.duration -= 1;
                                action.hitboxes.as_mut().unwrap()[0].duration =
                                    self.hitbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.duration += 1;
                                action.hitboxes.as_mut().unwrap()[0].duration =
                                    self.hitbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].duration =
                                    self.old_hitbox.duration;
                                self.hitbox.duration = self.old_hitbox.duration;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Hitstop"));
                            let hitstop = &mut (self.hitbox.properties.hitstop as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, hitstop, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.hitstop -= 1;
                                action.hitboxes.as_mut().unwrap()[0].duration =
                                    self.hitbox.properties.hitstop;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.hitstop += 1;
                                action.hitboxes.as_mut().unwrap()[0].properties.hitstop =
                                    self.hitbox.properties.hitstop;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].properties.hitstop =
                                    self.old_hitbox.properties.hitstop;
                                self.hitbox.properties.hitstop = self.old_hitbox.properties.hitstop;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Hitstun"));
                            let hitstun = &mut (self.hitbox.properties.hitstun as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, hitstun, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.hitstun -= 1;
                                action.hitboxes.as_mut().unwrap()[0].properties.hitstun =
                                    self.hitbox.properties.hitstun;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.hitstun += 1;
                                action.hitboxes.as_mut().unwrap()[0].properties.hitstun =
                                    self.hitbox.properties.hitstun;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].properties.hitstun =
                                    self.old_hitbox.properties.hitstun;
                                self.hitbox.properties.hitstun = self.old_hitbox.properties.hitstun;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Blockstun"));
                            let blckstn = &mut (self.hitbox.properties.blockstun as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, blckstn, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.blockstun -= 1;
                                action.hitboxes.as_mut().unwrap()[0].properties.blockstun =
                                    self.hitbox.properties.blockstun;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.blockstun += 1;
                                action.hitboxes.as_mut().unwrap()[0].properties.blockstun =
                                    self.hitbox.properties.blockstun;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].properties.blockstun =
                                    self.old_hitbox.properties.blockstun;
                                self.hitbox.properties.blockstun =
                                    self.old_hitbox.properties.blockstun;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Knockback"));
                            let kb = &mut self.hitbox.properties.knockback;
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, kb, 1, 99900, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hitbox.properties.knockback -= 100;
                                action.hitboxes.as_mut().unwrap()[0].properties.knockback =
                                    self.hitbox.properties.knockback;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.properties.knockback += 100;
                                action.hitboxes.as_mut().unwrap()[0].properties.knockback =
                                    self.hitbox.properties.knockback;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].properties.knockback =
                                    self.old_hitbox.properties.knockback;
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
                                action.hitboxes.as_mut().unwrap()[0].value.top =
                                    self.hitbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.top += 1000;
                                action.hitboxes.as_mut().unwrap()[0].value.top =
                                    self.hitbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].value.top =
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
                                action.hitboxes.as_mut().unwrap()[0].value.bottom =
                                    self.hitbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.bottom += 1000;
                                action.hitboxes.as_mut().unwrap()[0].value.bottom =
                                    self.hitbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].value.bottom =
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
                                action.hitboxes.as_mut().unwrap()[0].value.left =
                                    self.hitbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.left += 1000;
                                action.hitboxes.as_mut().unwrap()[0].value.left =
                                    self.hitbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].value.left =
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
                                action.hitboxes.as_mut().unwrap()[0].value.right =
                                    self.hitbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hitbox.value.right += 1000;
                                action.hitboxes.as_mut().unwrap()[0].value.right =
                                    self.hitbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hitboxes.as_mut().unwrap()[0].value.right =
                                    self.old_hitbox.value.right;
                                self.hitbox.value.right = self.old_hitbox.value.right;
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
                                action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                    self.hurtbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                self.hurtbox.start_frame += 1;
                                action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                    self.hurtbox.start_frame;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                    self.old_hitbox.start_frame;
                                self.hurtbox.start_frame = self.old_hitbox.start_frame;
                            }

                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                            let dur = &mut (self.hurtbox.duration as i32);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.duration -= 1;
                                action.hurtboxes.as_mut().unwrap()[0].duration =
                                    self.hurtbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.duration += 1;
                                action.hurtboxes.as_mut().unwrap()[0].duration =
                                    self.hurtbox.duration;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].duration =
                                    self.old_hitbox.duration;
                                self.hurtbox.duration = self.old_hitbox.duration;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                            let top = &mut self.hurtbox.value.top;
                            let top = &mut world_to_screen_num(*top);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.top -= 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.top =
                                    self.hurtbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.top += 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.top =
                                    self.hurtbox.value.top;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].value.top =
                                    self.old_hitbox.value.top;
                                self.hurtbox.value.top = self.old_hitbox.value.top;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                            let bottom = &mut self.hurtbox.value.bottom;
                            let bottom = &mut world_to_screen_num(*bottom);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, bottom, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.bottom -= 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                    self.hurtbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.bottom += 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                    self.hurtbox.value.bottom;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                    self.old_hitbox.value.bottom;
                                self.hurtbox.value.bottom = self.old_hitbox.value.bottom;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                            let left = &mut self.hurtbox.value.left;
                            let left = &mut world_to_screen_num(*left);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, left, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.left -= 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.left =
                                    self.hurtbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.left += 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.left =
                                    self.hurtbox.value.left;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].value.left =
                                    self.old_hitbox.value.left;
                                self.hurtbox.value.left = self.old_hitbox.value.left;
                            }

                            let z = z + 12;
                            d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                            let right = &mut self.hurtbox.value.right;
                            let right = &mut world_to_screen_num(*right);
                            d.gui_value_box(rrect(x2, y + z, 30, 8), None, right, 1, 1000, false);
                            if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                self.hurtbox.value.right -= 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.right =
                                    self.hurtbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                self.hurtbox.value.right += 1000;
                                action.hurtboxes.as_mut().unwrap()[0].value.right =
                                    self.hurtbox.value.right;
                            }
                            if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset")) {
                                action.hurtboxes.as_mut().unwrap()[0].value.right =
                                    self.old_hitbox.value.right;
                                self.hurtbox.value.right = self.old_hitbox.value.right;
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
                                    action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                        self.pushbox.start_frame;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y, 10, 8), Some(c"+")) {
                                    self.pushbox.start_frame += 1;
                                    action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                        self.pushbox.start_frame;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y, 40, 8), Some(c"reset")) {
                                    action.hurtboxes.as_mut().unwrap()[0].start_frame =
                                        self.old_hitbox.start_frame;
                                    self.pushbox.start_frame = self.old_hitbox.start_frame;
                                }

                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Duration"));
                                let dur = &mut (self.pushbox.duration as i32);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, dur, 1, 100, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.duration -= 1;
                                    action.hurtboxes.as_mut().unwrap()[0].duration =
                                        self.pushbox.duration;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.duration += 1;
                                    action.hurtboxes.as_mut().unwrap()[0].duration =
                                        self.pushbox.duration;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.hurtboxes.as_mut().unwrap()[0].duration =
                                        self.old_hitbox.duration;
                                    self.pushbox.duration = self.old_hitbox.duration;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Top"));
                                let top = &mut self.pushbox.value.top;
                                let top = &mut world_to_screen_num(*top);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, top, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.top -= 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.top =
                                        self.pushbox.value.top;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.top += 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.top =
                                        self.pushbox.value.top;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.hurtboxes.as_mut().unwrap()[0].value.top =
                                        self.old_hitbox.value.top;
                                    self.pushbox.value.top = self.old_hitbox.value.top;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Bottom"));
                                let bottom = &mut self.pushbox.value.bottom;
                                let btm = &mut world_to_screen_num(*bottom);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, btm, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.bottom -= 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                        self.pushbox.value.bottom;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.bottom += 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                        self.pushbox.value.bottom;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.hurtboxes.as_mut().unwrap()[0].value.bottom =
                                        self.old_hitbox.value.bottom;
                                    self.pushbox.value.bottom = self.old_hitbox.value.bottom;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Left"));
                                let left = &mut self.pushbox.value.left;
                                let lft = &mut world_to_screen_num(*left);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, lft, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.left -= 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.left =
                                        self.pushbox.value.left;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.left += 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.left =
                                        self.pushbox.value.left;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.hurtboxes.as_mut().unwrap()[0].value.left =
                                        self.old_hitbox.value.left;
                                    self.pushbox.value.left = self.old_hitbox.value.left;
                                }

                                let z = z + 12;
                                d.gui_label(rrect(x, y + z, 60, 8), Some(c"Right"));
                                let right = &mut self.pushbox.value.right;
                                let rgt = &mut world_to_screen_num(*right);
                                d.gui_value_box(rrect(x2, y + z, 30, 8), None, rgt, 1, 1000, false);
                                if d.gui_label_button(rrect(x2 + 34, y + z, 10, 8), Some(c"-")) {
                                    self.pushbox.value.right -= 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.right =
                                        self.pushbox.value.right;
                                }
                                if d.gui_label_button(rrect(x2 + 46, y + z, 10, 8), Some(c"+")) {
                                    self.pushbox.value.right += 1000;
                                    action.hurtboxes.as_mut().unwrap()[0].value.right =
                                        self.pushbox.value.right;
                                }
                                if d.gui_label_button(rrect(x2 + 60, y + z, 40, 8), Some(c"reset"))
                                {
                                    action.hurtboxes.as_mut().unwrap()[0].value.right =
                                        self.old_hitbox.value.right;
                                    self.pushbox.value.right = self.old_hitbox.value.right;
                                }
                            }
                        }
                    }
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
    StLightPunch,
    StMediumPunch,
    StHeavyPunch,
    StLightKick,
    StMediumKick,
    StHeavyKick,
    CrLightPunch,
    CrMediumPunch,
    CrHeavyPunch,
    CrLightKick,
    CrMediumKick,
    CrHeavyKick,
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
            State::StLightPunch => "St LightPunch".to_string(),
            State::StMediumPunch => "St MediumPunch".to_string(),
            State::StHeavyPunch => "St HeavyPunch".to_string(),
            State::StLightKick => "St LightKick".to_string(),
            State::StMediumKick => "St MediumKick".to_string(),
            State::StHeavyKick => "St HeavyKick".to_string(),
            State::CrLightPunch => "Cr LightPunch".to_string(),
            State::CrMediumPunch => "Cr MediumPunch".to_string(),
            State::CrHeavyPunch => "Cr HeavyPunch".to_string(),
            State::CrLightKick => "Cr LightKick".to_string(),
            State::CrMediumKick => "Cr MediumKick".to_string(),
            State::CrHeavyKick => "Cr HeavyKick".to_string(),
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
    Hitbox,
    Hurtbox,
    Pushbox,
}

impl Property {
    pub fn name(&self) -> String {
        match self {
            Property::Hitbox => "Hitbox".to_string(),
            Property::Hurtbox => "Hurtbox".to_string(),
            Property::Pushbox => "Pushbox".to_string(),
        }
    }
}
