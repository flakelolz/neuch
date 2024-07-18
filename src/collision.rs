use crate::prelude::*;

#[derive(Clone, Default)]
pub struct Collisions {
    pub proximity: Vec<(Entity, ProximityBox)>,
    pub hitboxes: Vec<(Entity, Hitbox)>,
    pub hurtboxes: Vec<(Entity, Hurtbox)>,
    pub pushboxes: Vec<(Entity, Pushbox)>,
}

impl Collisions {
    pub fn update(&mut self, world: &mut World, hit_events: &mut Vec<HitEvent>) {
        self.store(world);
        self.check(world, hit_events);
        self.clear();
    }
    pub fn store(&mut self, world: &mut World) {
        for (id, (character, physics, state)) in
            world.query_mut::<(&Character, &Physics, &StateMachine)>()
        {
            let offset = physics.position;

            if let Some(action) = find_action(character, &state.processor.current.name()) {
                if let Some(modifiers) = &action.modifiers {
                    if let Some(proximity) = modifiers.proximity {
                        let proximity = proximity.translated(offset, physics.facing_left);
                        if proximity.is_active(state.context.elapsed) {
                            self.proximity.push((id, proximity));
                        }
                    }
                }
                if let Some(hitboxes) = &action.hitboxes {
                    for hitbox in hitboxes.iter() {
                        let hitbox = hitbox.translated(offset, physics.facing_left);
                        if hitbox.is_active(state.context.elapsed) {
                            self.hitboxes.push((id, hitbox));
                        }
                    }
                }
                if let Some(hurtboxes) = &action.hurtboxes {
                    for hurtbox in hurtboxes.iter() {
                        let hurtbox = hurtbox.translated(offset, physics.facing_left);
                        if hurtbox.is_active(state.context.elapsed) {
                            self.hurtboxes.push((id, hurtbox));
                        }
                    }
                }
                if let Some(pushboxes) = &action.pushboxes {
                    for pushbox in pushboxes.iter() {
                        let pushbox = pushbox.translated(offset, physics.facing_left);
                        if pushbox.is_active(state.context.elapsed) {
                            self.pushboxes.push((id, pushbox));
                        }
                    }
                } else {
                    let pushbox = if physics.facing_left {
                        character.info.pushbox.translate_flipped(offset)
                    } else {
                        character.info.pushbox.translate(offset)
                    };
                    self.pushboxes.push((
                        id,
                        Pushbox {
                            start_frame: 0,
                            duration: 1,
                            value: pushbox,
                        },
                    ));
                }
            }
        }
    }
    fn check(&self, world: &mut World, hit_events: &mut Vec<HitEvent>) {
        for (id, proximity) in self.proximity.iter() {
            for (defender, hurtbox) in self.hurtboxes.iter() {
                if boxes_overlap(&proximity.value, &hurtbox.value) {
                    hit_events.push(HitEvent {
                        attacker: *id,
                        defender: *defender,
                        proximity: Some(*proximity),
                        properties: HitProperties {
                            blockstun: proximity.duration,
                            ..Default::default()
                        },
                    });
                }
            }
        }

        for (attacker, hitbox) in self.hitboxes.iter() {
            for (defender, hurtbox) in self.hurtboxes.iter() {
                if attacker != defender && boxes_overlap(&hitbox.value, &hurtbox.value) {
                    let has_hit = &mut world
                        .get::<&mut StateMachine>(*attacker)
                        .unwrap()
                        .context
                        .reaction
                        .has_hit;

                    if *has_hit {
                        continue;
                    }

                    *has_hit = true;
                    hit_events.push(HitEvent {
                        attacker: *attacker,
                        defender: *defender,
                        properties: HitProperties {
                            hit_type: hitbox.properties.hit_type,
                            reaction_type: hitbox.properties.reaction_type,
                            hitstop: hitbox.properties.hitstop,
                            hitstun: hitbox.properties.hitstun,
                            blockstun: hitbox.properties.blockstun,
                            knockback: hitbox.properties.knockback,
                        },
                        proximity: None,
                    });
                }
            }
        }

        let mut distance;
        for (attacker, a_pushbox) in self.pushboxes.iter() {
            for (defender, b_pushbox) in self.pushboxes.iter() {
                if attacker != defender && boxes_overlap(&a_pushbox.value, &b_pushbox.value) {
                    let left = a_pushbox.value.left.max(b_pushbox.value.left);
                    let right = a_pushbox.value.right.min(b_pushbox.value.right);
                    distance = right - left;
                    // println!("left: {} right: {} distance: {}", left, right, distance);
                    let half = distance / 2;
                    let mut players = world
                        .query_mut::<&mut Physics>()
                        .into_iter()
                        .collect::<Vec<_>>();
                    let split = &mut players.split_at_mut(1);
                    let (p1, p2) = split;
                    if let Some((_, a_physics)) = p1.get_mut(0) {
                        if let Some((_, b_physics)) = p2.get_mut(0) {
                            if a_physics.position.x < b_physics.position.x {
                                a_physics.position.x -= half;
                                b_physics.position.x += half;
                            } else {
                                a_physics.position.x += half;
                                b_physics.position.x -= half;
                            }
                        }
                    }
                }
            }
        }
    }

    fn clear(&mut self) {
        self.proximity.clear();
        self.hitboxes.clear();
        self.hurtboxes.clear();
        self.pushboxes.clear();
    }
}

impl Hitbox {
    /// Check if a hitbox is active on a specified frame
    pub fn is_active(&self, frame: u32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }

    pub fn translated(&self, offset: IVec2, flipped: bool) -> Self {
        Self {
            value: if flipped {
                self.value.translate_flipped(offset)
            } else {
                self.value.translate(offset)
            },
            ..*self
        }
    }
}

impl ProximityBox {
    pub fn is_active(&self, frame: u32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }

    pub fn translated(&self, offset: IVec2, flipped: bool) -> Self {
        Self {
            value: if flipped {
                self.value.translate_flipped(offset)
            } else {
                self.value.translate(offset)
            },
            ..*self
        }
    }
}

impl Hurtbox {
    /// Check if a hitbox is active on a specified frame
    pub fn is_active(&self, frame: u32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }

    pub fn translated(&self, offset: IVec2, flipped: bool) -> Self {
        Self {
            value: if flipped {
                self.value.translate_flipped(offset)
            } else {
                self.value.translate(offset)
            },
            ..*self
        }
    }
}

impl Pushbox {
    pub fn is_active(&self, frame: u32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }
    pub fn translated(&self, offset: IVec2, flipped: bool) -> Self {
        Self {
            value: if flipped {
                self.value.translate_flipped(offset)
            } else {
                self.value.translate(offset)
            },
            ..*self
        }
    }
}

impl Boxes {
    /// Translate the hitbox by an offset
    pub fn translate(&self, offset: IVec2) -> Self {
        Self {
            left: self.left + offset.x,
            right: self.right + offset.x,
            top: self.top + offset.y,
            bottom: self.bottom + offset.y,
        }
    }
    /// Translate the hitbox by an offset when facing left
    pub fn translate_flipped(&self, offset: IVec2) -> Self {
        Self {
            left: -self.right + offset.x,
            right: -self.left + offset.x,
            top: self.top + offset.y,
            bottom: self.bottom + offset.y,
        }
    }
}

/// Check if two boxes overlap
fn boxes_overlap(a: &Boxes, b: &Boxes) -> bool {
    !((a.left > b.right) || (b.left > a.right) || (a.bottom > b.top) || (b.bottom > a.top))
}
