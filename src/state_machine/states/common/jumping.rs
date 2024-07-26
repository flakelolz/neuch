use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> String {
        "Jmp Start".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp Start on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Set jump direction
        if context.ctx.flags.jump == JumpFlags::None {
            handle_jump_flags(&mut context.ctx, buffer, physics);
        }
        // Base case
        if context.elapsed > context.duration {
            // Transitions
            match context.ctx.flags.jump {
                JumpFlags::None => context.ctx.next = Some(Box::new(jumping::Neutral)),
                JumpFlags::Forward => context.ctx.next = Some(Box::new(jumping::Forward)),
                JumpFlags::Backward => context.ctx.next = Some(Box::new(jumping::Backward)),
            }
        }
    }

    fn on_exit(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        context.ctx.flags.jump = JumpFlags::None;
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
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, buffer, physics) {
            return;
        }
        // Transitions
        attack_transitions(context, buffer, physics);
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
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            physics.set_forward_velocity(context.character.unwrap_or_default().jump_forward);
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, buffer, physics) {
            return;
        }
        // Transitions
        attack_transitions(context, buffer, physics);
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
            physics.velocity.y = context.character.unwrap_or_default().jump_velocity;
            physics.acceleration.y = context.character.unwrap_or_default().jump_deceleration;
            physics.set_forward_velocity(context.character.unwrap_or_default().jump_backward);
            context.ctx.airborne = true;
        }
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        if handle_ground_collision(context, buffer, physics) {
            return;
        }
        // Transitions
        attack_transitions(context, buffer, physics);
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
        // Base case
        if context.elapsed > context.duration {
            // Transitions
            if attack_transitions(context, buffer, physics) {
                return;
            }
            if jump_transitions(context, buffer, physics) {
                return;
            }
            if crouch_transition(context, buffer, physics) {
                return;
            }
            if walk_transition(context, buffer, physics) {
                return;
            }
            // return to idle
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp End on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "Jmp LightPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "Jmp MediumPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "Jmp HeavyPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp HeavyPunch on_exit");
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> String {
        "Jmp LightKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp LightKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp LightKick on_exit");
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> String {
        "Jmp MediumKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp MediumKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp MediumKick on_exit");
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> String {
        "Jmp HeavyKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp HeavyKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_jumping_attack_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp HeavyKick on_exit");
    }
}

pub struct AttackEnd;
impl State for AttackEnd {
    fn name(&self) -> String {
        "Jmp AttackEnd".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp AttackEnd on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        // Base case
        handle_ground_collision(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Jmp AttackEnd on_exit");
    }
}
