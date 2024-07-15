use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Reaction {
    pub has_hit: bool,
    pub hitstop: u32,
    pub hitstun: u32,
    pub blockstun: u32,
    pub knockback: IVec2,
    pub air_knockback: IVec2,
}

#[derive(Clone)]
pub struct Collisions {
    pub hitboxes: Vec<(Entity, Hitbox)>,
    pub hurtboxes: Vec<(Entity, Hurtbox)>,
    pub hit_event: Entity,
}

impl Collisions {
    pub fn new(hit_event: Entity) -> Self {
        Self {
            hitboxes: Vec::new(),
            hurtboxes: Vec::new(),
            hit_event,
        }
    }
    pub fn update(&mut self, world: &mut World) {
        self.store(world);
        self.check(world);
        self.clear();
    }
    pub fn store(&mut self, world: &mut World) {
        for (id, (character, physics, state)) in
            world.query_mut::<(&Character, &Physics, &StateMachine)>()
        {
            let offset = physics.position;

            if let Some(action) = find_action(character, &state.processor.current.name()) {
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
            }
        }
    }

    fn check(&self, world: &mut World) {
        for (attacker, hitbox) in self.hitboxes.iter() {
            for (defender, hurtbox) in self.hurtboxes.iter() {
                if attacker != defender && boxes_overlap(&hitbox.value, &hurtbox.value) {
                    let mut hit_event = world.get::<&mut Vec<HitEvent>>(self.hit_event).unwrap();
                    hit_event.push(HitEvent {
                        attacker: *attacker,
                        defender: *defender,
                        properties: HitProperties {
                            hitstop: hitbox.properties.hitstop,
                            hitstun: hitbox.properties.hitstun,
                            blockstun: hitbox.properties.blockstun,
                            knockback: hitbox.properties.knockback,
                            air_knockback: hitbox.properties.air_knockback,
                        },
                    });
                    // println!(
                    //     "Attacker: {:?} overlaps with Defender: {:?}",
                    //     attacker, defender
                    // );
                    // println!("Hit Event: {:#?}", &hit_event);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.hitboxes.clear();
        self.hurtboxes.clear();
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
            top: self.top + offset.y,
            right: -self.left + offset.x,
            bottom: self.bottom + offset.y,
        }
    }
}

/// Check if two boxes overlap
fn boxes_overlap(a: &Boxes, b: &Boxes) -> bool {
    !((a.left > b.right) || (b.left > a.right) || (a.bottom > b.top) || (b.bottom > a.top))
}
