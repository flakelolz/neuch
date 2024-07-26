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
        // Apply physics and handle modifiers
        physics.velocity.x = 0;
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if crouch_transition(context, buffer, physics) {
            return;
        }
        if dash_transitions(context, buffer, physics) {
            return;
        }
        walk_transition(context, buffer, physics);
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

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("WalkForward on_enter");
        // FIX: Find a way to move on the first frame
        physics.set_forward_velocity(context.character.unwrap_or_default().walk_forward);
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Special case for walking
        physics.set_forward_velocity(context.character.unwrap_or_default().walk_forward);
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if crouch_transition(context, buffer, physics) {
            return;
        }
        if dash_transitions(context, buffer, physics) {
            return;
        }
        // Base case & return to idle
        if !forward(buffer, &physics.facing_left) {
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        // If velocity was applied earlier in the state, remove it
        physics.velocity.x = 0;
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> String {
        "St WalkBackward".to_owned()
    }

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("WalkBackward on_enter");
        // FIX: Find a way to move on the first frame
        physics.set_forward_velocity(context.character.unwrap_or_default().walk_backward);
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Special case for walking
        physics.set_forward_velocity(context.character.unwrap_or_default().walk_backward);
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if crouch_transition(context, buffer, physics) {
            return;
        }
        // Base case & return to idle
        if !backward(buffer, &physics.facing_left) {
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        // If velocity was applied earlier in the state, remove it
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
        // Base case
        if context.elapsed > context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
                return;
            }
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
            // Return to idle
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
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
        // Base case
        if context.elapsed > context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
                return;
            }
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
            // Return to idle
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_attack_transitions(context, buffer, physics);
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
        common_standing_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyKick on_exit");
    }
}

pub struct Turn;
impl State for Turn {
    fn name(&self) -> String {
        "St Turn".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St Turn on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Transitions
        if context.elapsed > context.duration {
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
            // Return to idle
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St Turn on_exit");
    }
}
