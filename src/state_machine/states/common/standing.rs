use crate::prelude::*;

pub struct Idle;
impl State for Idle {
    fn name(&self) -> String {
        "St Idle".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St Idle on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;

        let input = &buffer.get_curret_input();

        if dash_transitions(context, buffer) {
            return;
        }

        if crouch_attack_transitions(context, input) {
            return;
        }

        if attack_transitions(context, input) {
            return;
        }

        if input.down {
            context.next = Some(Box::new(crouching::Start));
        }

        walk_transition(context, buffer);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St Idle on_exit");
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> String {
        "St WalkForward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("WalkForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 3000;

        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> String {
        "St WalkBackward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = -3000;

        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("WalkBackward on_exit");
    }
}

pub struct DashForward;
impl State for DashForward {
    fn name(&self) -> String {
        "St DashForward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("DashForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        let input = &buffer.get_curret_input();

        handle_modifiers(context, buffer, physics);

        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Start));
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("DashForward on_exit");
    }
}

pub struct DashBackward;
impl State for DashBackward {
    fn name(&self) -> String {
        "St DashBackward".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("DashBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        let input = &buffer.get_curret_input();

        handle_modifiers(context, buffer, physics);

        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Start));
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("DashBackward on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "St LightPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "St MediumPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        let input = &buffer.get_curret_input();

        if context.elapsed >= context.duration - 1 {
            if dash_transitions(context, buffer) {
                return;
            }

            if input.down && crouch_attack_transitions(context, input) {
                return;
            }

            if attack_transitions(context, input) {
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "St HeavyPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyPunch on_exit");
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> String {
        "St LightKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St LightKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St LightKick on_exit");
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> String {
        "St MediumKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("MediumKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        let input = &buffer.get_curret_input();

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("MediumKick on_exit");
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> String {
        "St HeavyKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        let input = &buffer.get_curret_input();

        handle_modifiers(context, buffer, physics);

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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyKick on_exit");
    }
}
