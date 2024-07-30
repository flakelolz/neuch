use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Ken {
    Normals,
    Specials,
}

impl Ken {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Ken::Normals => false,
            Ken::Specials => {
                if Specials::ForcedHadouken.set(buffer, ctx, physics) {
                    return true;
                }
                if Specials::ShoryukenL.set(buffer, ctx, physics) {
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
    ClLightPunch,
    ClMediumPunch,
    ClHeavyPunch,
    BckMediumKick,
    FwdMediumKick,
    FwdHeavyKick,
}

impl Normals {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Normals::ClLightPunch => todo!(),
            Normals::ClMediumPunch => todo!(),
            Normals::ClHeavyPunch => todo!(),
            Normals::BckMediumKick => todo!(),
            Normals::FwdMediumKick => todo!(),
            Normals::FwdHeavyKick => todo!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Specials {
    Hadouken,
    ForcedHadouken,
    ShoryukenL,
}

impl Specials {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        let flipped = &physics.facing_left;
        match self {
            Specials::Hadouken => {
                if buffer.was_motion_executed(Motions::Qcf, Inputs::LightPunch, flipped)
                    && buffer.buffered(Inputs::LightPunch, buffer.attack, flipped)
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(Hadouken));

                    return true;
                }

                false
            }
            Specials::ForcedHadouken => {
                if buffer.was_motion_executed(Motions::Hcf, Inputs::LightPunch, flipped)
                    && buffer.buffered(Inputs::LightPunch, buffer.attack, flipped)
                    && !physics.airborne
                {
                    ctx.next.replace(Box::new(Hadouken));

                    return true;
                }
                false
            }
            Specials::ShoryukenL => {
                if buffer.was_motion_executed(Motions::Dpf, Inputs::LightPunch, flipped)
                    && buffer.buffered(Inputs::LightPunch, buffer.attack, flipped)
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

pub struct Hadouken;
impl State for Hadouken {
    fn name(&self) -> String {
        "Ken Hadouken".into()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Ken Hadouken on_enter");
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

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("Ken ShoryukenL on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
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
