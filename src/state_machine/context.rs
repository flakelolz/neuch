use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub next: Option<Box<dyn State>>,
    pub elapsed: i32,
    pub duration: i32,
    pub modifier: Instructions,
    pub locked: LockedActions,
}

// Naming is hard
#[derive(Default)]
pub struct Instructions {
    pub index: usize,
    pub instructions: Option<Modifiers>,
}

pub fn handle_modifiers(context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
    if let Some(instructions) = &context.modifier.instructions {
        if let Some(position) = &instructions.potisions {
            if let Some(position) = position.get(context.modifier.index) {
                if position.on_frame == context.elapsed {
                    physics.position += position.value;
                    context.modifier.index += 1;
                }
            }
        }

        if let Some(chainable) = &instructions.chainable {
            if chainable.on_frame - 1 == context.elapsed {
                let input = &buffer.get_curret_input();
                if chainable.st_lk && !input.down && input.lk {
                    context.next = Some(Box::new(standing::LightKick));
                    return;
                }
                if chainable.cr_lk && input.down && input.lk {
                    context.next = Some(Box::new(crouching::LightKick));
                    return;
                }
                if chainable.st_lp && !input.down && input.lp {
                    context.next = Some(Box::new(standing::LightPunch));
                    return;
                }
                if chainable.cr_lp && input.down && input.lp {
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

impl LockedActions {
    pub fn check_valid(&mut self, buffer: &InputBuffer) {
        self.dash_forward(buffer);
        self.dash_backward(buffer);
    }

    fn dash_forward(&mut self, buffer: &InputBuffer) {
        let time = 8;
        if buffer.is_input_held(&Inputs::Forward, time)
            && !buffer.is_input_held(&Inputs::DownForward, time)
        {
            self.dash_forward = false;
        } else if buffer.is_input_held(&Inputs::Neutral, time)
            || buffer.is_input_held(&Inputs::Backward, time)
            || buffer.is_input_held(&Inputs::Down, time)
            || buffer.is_input_held(&Inputs::Up, time)
        {
            self.dash_forward = true;
        }
    }

    fn dash_backward(&mut self, buffer: &InputBuffer) {
        let time = 8;
        if buffer.is_input_held(&Inputs::Backward, time)
            && !buffer.is_input_held(&Inputs::DownBackward, time)
        {
            self.dash_backward = false;
        } else if buffer.is_input_held(&Inputs::Neutral, time)
            || buffer.is_input_held(&Inputs::Forward, time)
            || buffer.is_input_held(&Inputs::Down, time)
            || buffer.is_input_held(&Inputs::Up, time)
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
