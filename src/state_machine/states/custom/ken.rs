use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Ken {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    Normals,
    Specials,
}

impl Ken {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Ken::Normals => {
                if Normals::HeavyPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Normals::MediumPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Normals::LightPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Ken::HeavyPunch => {
                if Normals::HeavyPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Ken::MediumPunch => {
                if Normals::MediumPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Ken::LightPunch => {
                if Normals::LightPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Ken::Specials => {
                // Priority Hadouken with half-circle motion
                {
                    let lp = Inputs::LightPunch;
                    let mp = Inputs::MediumPunch;
                    let hp = Inputs::HeavyPunch;
                    let hcf = [4, 1, 2, 3, 6];
                    if (buffer.was_motion_executed_exact(&hcf, lp)
                        || buffer.was_motion_executed_exact(&hcf, mp)
                        || buffer.was_motion_executed_exact(&hcf, hp))
                        && (buffer.buffered(lp, buffer.cancels, &physics.facing_left)
                            || buffer.buffered(mp, buffer.cancels, &physics.facing_left)
                            || buffer.buffered(hp, buffer.cancels, &physics.facing_left))
                        && !physics.airborne
                    {
                        ctx.next.replace(Box::new(Hadouken));

                        return true;
                    }
                }
                if Specials::Shoryuken.set(buffer, ctx, physics) {
                    return true;
                }
                if Specials::Hadouken.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Normals {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    BckMediumKick,
    FwdMediumKick,
    FwdHeavyKick,
}

impl Normals {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        let distance = world_to_screen_num(physics.distance as i32);
        match self {
            Normals::LightPunch => {
                if buffer.buffered(Inputs::LightPunch, buffer.attack, &physics.facing_left)
                    && distance < 35
                {
                    ctx.next.replace(Box::new(ken::LightPunch));
                    return true;
                }
                false
            }
            Normals::MediumPunch => {
                if buffer.buffered(Inputs::MediumPunch, buffer.attack, &physics.facing_left)
                    && distance < 40
                {
                    ctx.next.replace(Box::new(ken::MediumPunch));
                    return true;
                }
                false
            }
            Normals::HeavyPunch => {
                if buffer.buffered(Inputs::HeavyPunch, buffer.attack, &physics.facing_left)
                    && distance < 48
                {
                    ctx.next.replace(Box::new(ken::HeavyPunch));
                    return true;
                }
                false
            }
            Normals::BckMediumKick => todo!(),
            Normals::FwdMediumKick => todo!(),
            Normals::FwdHeavyKick => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Specials {
    Hadouken,
    Shoryuken,
}

impl Specials {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        let flipped = &physics.facing_left;
        match self {
            Specials::Hadouken => {
                let lp = Inputs::LightPunch;
                let mp = Inputs::MediumPunch;
                let hp = Inputs::HeavyPunch;
                if (buffer.was_motion_executed(Motions::Qcf, lp)
                    || buffer.was_motion_executed(Motions::Qcf, mp)
                    || buffer.was_motion_executed(Motions::Qcf, hp))
                    && (buffer.buffered(lp, buffer.cancels, flipped)
                        || buffer.buffered(mp, buffer.cancels, flipped)
                        || buffer.buffered(hp, buffer.cancels, flipped))
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(Hadouken));

                    return true;
                }
                false
            }
            Specials::Shoryuken => {
                if buffer.was_motion_executed(Motions::Dpf, Inputs::LightPunch)
                    && buffer.buffered(Inputs::LightPunch, 20, flipped)
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(ShoryukenL));

                    return true;
                }
                false
            }
        }
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "Cl LightPunch".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "Cl MediumPunch".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed > 8
            && buffer.buffered(Inputs::HeavyPunch, buffer.cancels, &physics.facing_left)
        {
            context.ctx.next.replace(Box::new(HeavyPunch));
            return;
        }
        common_standing_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "Cl HeavyPunch".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cl HeavyPunch on_exit");
    }
}

pub struct Hadouken;
impl State for Hadouken {
    fn name(&self) -> String {
        "Ken Hadouken".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Ken Hadouken on_enter");
        // TODO: Set how fast a fireball is going to move based on the button currently pressed
        println!(
            "lp: {}, mp: {}, hp: {}",
            _buffer.current().lp,
            _buffer.current().mp,
            _buffer.current().hp
        );
        let offset = IVec2 {
            x: {
                if _physics.facing_left {
                    -70000
                } else {
                    70000
                }
            },
            y: 0,
        };
        let mut physics = Physics {
            position: _physics.position + offset,
            facing_left: _physics.facing_left,
            facing_opponent: _physics.facing_opponent,
            ..Default::default()
        };

        if _buffer.current().lp {
            physics.set_forward_velocity(3000);
        }
        if _buffer.current().mp {
            physics.set_forward_velocity(4000);
        }
        if _buffer.current().hp {
            physics.set_forward_velocity(5000);
        }
        _context.spawn.replace(Projectile {
            owner: None,
            name: "Obj Fireball".into(),
            physics,
            timing: 13,
            duration: 100,
        });
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed > context.duration {
            common_standing_attack_transitions(context, buffer, physics);
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Ken Hadouken on_exit");
    }
}

pub struct ShoryukenL;
impl State for ShoryukenL {
    fn name(&self) -> String {
        "Ken ShoryukenL".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Ken ShoryukenL on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed == 3 && context.ctx.reaction.hitstop == 0 {
            physics.position.x += 5000;
        }
        if context.elapsed == 5 {
            physics.velocity.y = 9000;
            physics.acceleration.y = -750;
            physics.set_forward_velocity(1000);
            physics.airborne = true;
        }

        if context.elapsed > 14 {
            physics.velocity.x = 0;
        }

        if context.elapsed > 29 {
            physics.position.y = 0;
            physics.velocity = IVec2::zero();
            physics.acceleration.y = 0;
            physics.airborne = false;
        }
        if context.elapsed == 30 {
            physics.position.x += 4000;
        }

        if context.elapsed > context.duration {
            common_standing_attack_transitions(context, buffer, physics);
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Ken ShoryukenL on_exit");
    }
}
