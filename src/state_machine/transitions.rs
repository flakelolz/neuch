use crate::prelude::*;

pub fn handle_transition(
    state: &mut StateMachine,
    buffer: &mut InputBuffer,
    physics: &mut Physics,
    character: &Character,
    animator: &mut Animator,
    collision: &mut Collisions,
) {
    // If there is a next state to transition to it
    if let Some(mut next) = state.context.ctx.next.take() {
        let processor = &mut state.processor;
        let context = &mut state.context;
        // Setup the next state and reset variables
        processor.current.on_exit(context, buffer, physics);
        context.elapsed = 1;
        next.on_enter(context, buffer, physics);
        processor.current = next;
        animator.reset();
        context.ctx.reaction.has_hit = false;
        context.ctx.reaction.can_cancel = false;
        collision.counter = 0;
        buffer.cancels = 2;

        let name = processor.current.name();
        if let Some(action) = find_action(character, &name) {
            // Character info
            // FIX: This only needs to be set once.
            if context.character.is_none() {
                context.ctx.name = character.name.clone();
                context.character = Some(character.info);
            }
            // Setup action data
            context.duration = action.total;
            // Setup animnation data
            if context.ctx.reaction.hit() {
                hit_animation(animator, context, &action.timeline);
            } else if context.ctx.reaction.block() {
                guard_animation(animator, context, &action.timeline);
            } else {
                animator.keyframes.clone_from(&action.timeline);
            }
            // Setup action modifiers if there are any
            match &action.modifiers {
                Some(_) => {
                    context.modifiers.index = 0;
                    context.modifiers.instructions.clone_from(&action.modifiers);
                }
                None => {
                    context.modifiers.instructions = None;
                }
            }
        }
    }
}

pub fn common_standing_attack_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) {
    // Kara-cancel
    if context.elapsed == 2 && specials_transitions(context, buffer, physics) {
        return;
    }
    // Base case
    if context.elapsed > context.duration {
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if specials_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
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

pub fn common_crouching_attack_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) {
    // Kara-cancel
    if context.elapsed == 2 && specials_transitions(context, buffer, physics) {
        return;
    }
    // Base case
    if context.elapsed > context.duration {
        // Transitions
        if jump_transitions(context, buffer, physics) {
            return;
        }
        if specials_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if !down(buffer) {
            if dash_transitions(context, buffer, physics) {
                return;
            }
            if walk_transition(context, buffer, physics) {
                return;
            }
            // Return to idle
            context.ctx.next = Some(Box::new(crouching::End));
        } else {
            context.ctx.next = Some(Box::new(crouching::Idle));
        }
    }
}

pub fn common_jumping_attack_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) {
    if handle_ground_collision(context, buffer, physics) {
        return;
    }
    // Base case
    if context.elapsed > context.duration {
        // Transitions
        context.ctx.next = Some(Box::new(jumping::AttackEnd));
    }
}

pub fn common_standing_reaction_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) {
    if context.elapsed > context.duration {
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if specials_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
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

pub fn common_crouching_reaction_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) {
    if context.elapsed > context.duration {
        // Transitions
        if turn_transition(&mut context.ctx, buffer, physics) {
            return;
        }
        if specials_transitions(context, buffer, physics) {
            return;
        }
        if attack_transitions(context, buffer, physics) {
            return;
        }
        if jump_transitions(context, buffer, physics) {
            return;
        }
        if buffer.current().down {
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
        context.ctx.next = Some(Box::new(standing::Idle));
    }
}

pub fn crouch_transition(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if Crouching::Start.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    false
}

pub fn walk_transition(context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) -> bool {
    if Group::Walks.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    false
}

pub fn dash_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if Group::Dashes.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    false
}

pub fn attack_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if !physics.airborne && Group::Normals.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    if physics.airborne && Group::AirNormals.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    false
}

pub fn jump_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if up(buffer) {
        handle_jump_flags(&mut context.ctx, buffer, physics);
        context.ctx.next = Some(Box::new(jumping::Start));
        return true;
    }
    false
}

pub fn handle_jump_flags(ctx: &mut SubContext, buffer: &InputBuffer, physics: &Physics) {
    if up_forward(buffer, &physics.facing_left) {
        ctx.flags.jump = JumpFlags::Forward;
    }
    if up_backward(buffer, &physics.facing_left) {
        ctx.flags.jump = JumpFlags::Backward;
    }
}

pub fn handle_ground_collision(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if physics.position.y <= 0 {
        physics.position.y = 0;
        physics.velocity = IVec2::zero();
        physics.acceleration.y = 0;
        physics.airborne = false;
        turn_transition(&mut context.ctx, buffer, physics);
        context.ctx.next = Some(Box::new(jumping::End));

        return true;
    }
    false
}

pub fn turn_transition(ctx: &mut SubContext, buffer: &InputBuffer, physics: &mut Physics) -> bool {
    if ctx.reaction.hitstop == 0 && face_opponent(physics) {
        // Base case
        if down(buffer) {
            ctx.next = Some(Box::new(crouching::Turn));
            return true;
        }
        ctx.next = Some(Box::new(standing::Turn));
        return true;
    }
    false
}

pub fn specials_transitions(
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
) -> bool {
    if Group::Specials.set(buffer, &mut context.ctx, physics) {
        return true;
    }
    false
}
