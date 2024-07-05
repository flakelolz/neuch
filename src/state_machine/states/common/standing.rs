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
        if attack_transitions(context, buffer) {
            return;
        }
        if crouch_transition(context, buffer) {
            return;
        }
        if dash_transitions(context, buffer) {
            return;
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
        // Apply physics and handle modifiers
        physics.velocity.x = context.character.unwrap_or_default().walk_forward;
        // Transitions
        if attack_transitions(context, buffer) {
            return;
        }
        if crouch_transition(context, buffer) {
            return;
        }
        if dash_transitions(context, buffer) {
            return;
        }
        // Special case for walking the opposite direction
        if buffer.input().backward {
            context.next = Some(Box::new(standing::WalkBackward));
            return;
        }
        // Base case & return to idle
        if !buffer.input().forward {
            context.next = Some(Box::new(standing::Idle));
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

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Apply physics and handle modifiers
        physics.velocity.x = context.character.unwrap_or_default().walk_backward;
        // Transitions
        if attack_transitions(context, buffer) {
            return;
        }
        if crouch_transition(context, buffer) {
            return;
        }
        // Special case for walking the opposite direction
        if buffer.input().forward {
            context.next = Some(Box::new(standing::WalkForward));
            return;
        }
        // Base case & return to idle
        if !buffer.input().backward {
            context.next = Some(Box::new(standing::Idle));
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
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
            context.next = Some(Box::new(standing::Idle));
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
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
            context.next = Some(Box::new(standing::Idle));
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
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
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

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
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
        // Apply physics and handle modifiers
        handle_modifiers(context, buffer, physics);
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if attack_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if dash_transitions(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // Return to idle
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("St HeavyKick on_exit");
    }
}
