use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> String {
        "Jmp Start".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Start on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, _physics: &mut Physics) {
        // Base case
        if context.elapsed >= context.duration {
            // Transitions
            if forward(buffer) {
                context.ctx.next = Some(Box::new(jumping::Forward));
                return;
            }
            if backward(buffer) {
                context.ctx.next = Some(Box::new(jumping::Backward));
                return;
            }

            context.ctx.next = Some(Box::new(jumping::Neutral));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Start on_exit");
    }
}

pub struct Neutral;

impl State for Neutral {
    fn name(&self) -> String {
        "Jmp Neutral".to_owned()
    }

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("Jmp Neutral on_enter");
        if physics.position.y <= 0 {
            // FIX: Both on_enter and on_update are being run on the same frame it seems to it
            // immediately finds the position.y at 0.
            physics.position.y += 1;
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, physics) {
            return;
        }

        if attack_transitions(context, buffer) {
            return;
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Neutral on_exit");
    }
}

pub struct Forward;
impl State for Forward {
    fn name(&self) -> String {
        "Jmp Forward".to_owned()
    }

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("Jmp Forward on_enter");
        if physics.position.y <= 0 {
            // FIX: Both on_enter and on_update are being run on the same frame it seems to it
            // immediately finds the position.y at 0.
            physics.position.y += 1;
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            physics.velocity.x = context.character.unwrap_or_default().jump_forward;
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, physics) {
            return;
        }
        // Transitions
        if attack_transitions(context, buffer) {
            return;
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Forward on_exit");
    }
}

pub struct Backward;
impl State for Backward {
    fn name(&self) -> String {
        "Jmp Backward".to_owned()
    }

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        println!("Jmp Backward on_enter");
        if physics.position.y <= 0 {
            // FIX: Both on_enter and on_update are being run on the same frame it seems to it
            // immediately finds the position.y at 0.
            physics.position.y += 1;
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            physics.velocity.x = context.character.unwrap_or_default().jump_backward;
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, physics) {
            return;
        }
        // Transitions
        if attack_transitions(context, buffer) {
            return;
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Backward on_exit");
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> String {
        "Jmp End".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp End on_enter");
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
            if jump_transitions(context, buffer) {
                return;
            }
            if crouch_transition(context, buffer) {
                return;
            }
            if walk_transition(context, buffer) {
                return;
            }
            // return to idle
            if context.elapsed >= context.duration {
                context.ctx.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp End on_exit");
    }
}
