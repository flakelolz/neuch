use crate::prelude::*;

pub struct Idle;
impl State for Idle {
    fn name(&self) -> String {
        "St Idle".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St Idle on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = 0;

        if input.down {
            if crouch_attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(crouching::Start));
        }

        if input.forward {
            context.next = Some(Box::new(standing::WalkForward));
        }

        if input.backward {
            context.next = Some(Box::new(standing::WalkBackward));
        }

        attack_transitions(context, input);
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St Idle on_exit");
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> String {
        "St WalkForward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = 3000;

        if input.down {
            if crouch_attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(crouching::Start));
        }

        if input.backward {
            context.next = Some(Box::new(standing::WalkBackward));
            return;
        }

        if attack_transitions(context, input) {
            return;
        }

        if !input.forward {
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> String {
        "St WalkBackward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = -3000;

        if input.down {
            if crouch_attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(crouching::Start));
        }

        if input.forward {
            context.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if attack_transitions(context, input) {
            return;
        }

        if !input.backward {
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("WalkBackward on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "St LightPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "St MediumPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "St HeavyPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St HeavyPunch on_exit");
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> String {
        "St LightKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightKick on_exit");
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> String {
        "St MediumKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("MediumKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("MediumKick on_exit");
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> String {
        "St HeavyKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St HeavyKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        handle_modifiers(context, input, physics);

        if context.elapsed >= context.duration - 1 {
            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St HeavyKick on_exit");
    }
}
