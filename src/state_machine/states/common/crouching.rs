use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> String {
        "Cr Start".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Start on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Transitions
        if attack_transitions(context, buffer, physics) {
            return;
        }
        // Special case for releasing down on crouch start
        if context.elapsed > context.duration && down(buffer) {
            context.ctx.next = Some(Box::new(crouching::Idle));
        }
        // Base case
        if !down(buffer) {
            context.ctx.next = Some(Box::new(crouching::End));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Start on_exit");
    }
}

pub struct Idle;
impl State for Idle {
    fn name(&self) -> String {
        "Cr Idle".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Idle on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        // Base case
        if !down(buffer) {
            context.ctx.next = Some(Box::new(crouching::End));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Idle on_exit");
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> String {
        "Cr End".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr End on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Transitions
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if crouch_transition(context, buffer, physics) {
            return;
        }
        if dash_transitions(context, buffer, physics) {
            return;
        }
        if walk_transition(context, buffer, physics) {
            return;
        }
        // Base case & return to idle
        if context.elapsed > context.duration {
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr End on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "Cr LightPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "Cr MediumPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "Cr HeavyPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr HeavyPunch on_exit");
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> String {
        "Cr LightKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr LightKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr LightKick on_exit");
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> String {
        "Cr MediumKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr MediumKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr MediumKick on_exit");
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> String {
        "Cr HeavyKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr HeavyKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr HeavyKick on_exit");
    }
}

pub struct Turn;
impl State for Turn {
    fn name(&self) -> String {
        "Cr Turn".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Turn on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Transitions
        if context.elapsed > context.duration {
            if attack_transitions(context, buffer, physics) {
                return;
            }
            if down(buffer) {
                context.ctx.next = Some(Box::new(crouching::Idle));
                return;
            }
            if dash_transitions(context, buffer, physics) {
                return;
            }
            if walk_transition(context, buffer, physics) {
                return;
            }
            // Return to idle
            if !down(buffer) {
                context.ctx.next = Some(Box::new(crouching::End));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Cr Turn on_exit");
    }
}
