use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub character: Option<CharacterInfo>,
    pub next: Option<Box<dyn State>>,
    pub elapsed: u32,
    pub duration: u32,
    pub modifiers: Instructions,
    pub locked: LockedActions,
}

// Naming is hard
#[derive(Default)]
pub struct Instructions {
    pub index: usize,
    pub instructions: Option<Modifiers>,
}

pub fn handle_modifiers(context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
    if let Some(instructions) = &context.modifiers.instructions {
        if let Some(position) = &instructions.potisions {
            if let Some(position) = position.get(context.modifiers.index) {
                if position.on_frame == context.elapsed {
                    physics.position += position.value;
                    context.modifiers.index += 1;
                }
            }
        }

        // FIX: On whiff or on impact properties are different
        if let Some(chainable) = &instructions.chainable {
            if chainable.on_frame <= context.elapsed {
                if chainable.st_lk
                    && !buffer.input().down
                    && buffer.buffered(&Inputs::LightKick, buffer.attack)
                {
                    context.next = Some(Box::new(standing::LightKick));
                    return;
                }
                if chainable.cr_lk
                    && buffer.input().down
                    && buffer.buffered(&Inputs::LightKick, buffer.attack)
                {
                    context.next = Some(Box::new(crouching::LightKick));
                    return;
                }
                if chainable.st_lp
                    && !buffer.input().down
                    && buffer.buffered(&Inputs::LightPunch, buffer.attack)
                {
                    context.next = Some(Box::new(standing::LightPunch));
                    return;
                }
                if chainable.cr_lp
                    && buffer.input().down
                    && buffer.buffered(&Inputs::LightPunch, buffer.attack)
                {
                    context.next = Some(Box::new(crouching::LightPunch));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LockedActions {
    pub dash_forward: bool,
    pub dash_backward: bool,
}

// FIX: This feels cursed
impl LockedActions {
    pub fn check_valid(&mut self, buffer: &InputBuffer) {
        self.dash_forward(buffer);
        self.dash_backward(buffer);
    }

    fn dash_forward(&mut self, buffer: &InputBuffer) {
        let time = 8;
        if buffer.held(&Inputs::Forward, time) && !buffer.held(&Inputs::DownForward, time) {
            self.dash_forward = false;
        } else if buffer.held(&Inputs::Neutral, time)
            || buffer.held(&Inputs::Backward, time)
            || buffer.held(&Inputs::Down, time)
            || buffer.held(&Inputs::Up, time)
        {
            self.dash_forward = true;
        }
    }

    fn dash_backward(&mut self, buffer: &InputBuffer) {
        let time = 8;
        if buffer.held(&Inputs::Backward, time) && !buffer.held(&Inputs::DownBackward, time) {
            self.dash_backward = false;
        } else if buffer.held(&Inputs::Neutral, time)
            || buffer.held(&Inputs::Forward, time)
            || buffer.held(&Inputs::Down, time)
            || buffer.held(&Inputs::Up, time)
        {
            self.dash_backward = true;
        }
    }
}

impl Default for LockedActions {
    fn default() -> Self {
        Self {
            dash_forward: true,
            dash_backward: true,
        }
    }
}
