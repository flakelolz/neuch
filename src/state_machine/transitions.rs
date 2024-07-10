use crate::prelude::*;

pub fn handle_transition(
    processor: &mut StateProcessor,
    context: &mut Context,
    buffer: &InputBuffer,
    physics: &mut Physics,
    character: &Character,
    animator: &mut Animator,
) {
    // If there is a next state to transition to it
    if let Some(mut next) = context.ctx.next.take() {
        // Setup the next state and reset variables
        processor.current.on_exit(context, buffer, physics);
        context.elapsed = 1;
        next.on_enter(context, buffer, physics);
        processor.current = next;
        animator.reset();

        let name = processor.current.name();
        if let Some(action) = find_action(character, &name) {
            // Character info
            context.character = Some(character.info);
            // Setup action data
            context.duration = action.total;
            // Setup animnation data
            animator.keyframes.clone_from(&action.timeline);
            // Setup action modifiers if there are any
            match &action.modifiers {
                Some(modifiers) => {
                    context.modifiers.index = 0;
                    context.modifiers.instructions = Some(modifiers.clone());
                }
                None => {
                    context.modifiers.instructions = None;
                }
            }
        }

        return;
    }

    let name = processor.current.name();
    let action = find_action(character, &name);

    match action {
        Some(action) => {
            // NOTE: Only needed at the start of the game right now.
            if animator.keyframes.is_empty() {
                animator.keyframes.clone_from(&action.timeline);
            }

            context.elapsed += 1;

            if context.elapsed > action.total && action.looping {
                context.elapsed = 1;
            }
        }
        None => {
            eprintln!("Action not found");
        }
    }
}

pub fn crouch_transition(context: &mut Context, buffer: &InputBuffer) -> bool {
    if Crouching::Start.set(buffer, &mut context.ctx) {
        return true;
    }

    false
}

pub fn walk_transition(context: &mut Context, buffer: &InputBuffer) -> bool {
    if Group::Walks.set(buffer, &mut context.ctx) {
        return true;
    }

    false
}

pub fn dash_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    if Group::Dashes.set(buffer, &mut context.ctx) {
        return true;
    }
    false
}

pub fn attack_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    if !context.ctx.airborne && Group::Normals.set(buffer, &mut context.ctx) {
        return true;
    }
    if context.ctx.airborne && Group::AirNormals.set(buffer, &mut context.ctx) {
        return true;
    }

    false
}

pub fn jump_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    if up(buffer) {
        handle_jump_flags(&mut context.ctx, buffer);
        context.ctx.next = Some(Box::new(jumping::Start));
        return true;
    }
    false
}

pub fn handle_jump_flags(ctx: &mut SubContext, buffer: &InputBuffer) {
    if up_forward(buffer) {
        ctx.flags.jump = JumpFlags::Forward;
    }
    if up_backward(buffer) {
        ctx.flags.jump = JumpFlags::Backward;
    }
}

pub fn handle_ground_collision(context: &mut Context, physics: &mut Physics) -> bool {
    if physics.position.y <= 0 {
        physics.position.y = 0;
        physics.velocity.y = 0;
        physics.velocity.x = 0;
        physics.acceleration.y = 0;
        context.ctx.airborne = false;
        context.ctx.next = Some(Box::new(jumping::End));
        return true;
    }
    false
}
