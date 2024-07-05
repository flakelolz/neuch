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
    if let Some(mut next) = context.next.take() {
        // Setup the next state and reset variables
        processor.current.on_exit(context, buffer, physics);
        context.elapsed = 0;
        next.on_enter(context, buffer, physics);
        processor.current = next;
        animator.reset();

        let name = processor.current.name();
        if let Some(action) = find_action(character, &name) {
            context.duration = action.total;
            // Setup animnation data
            animator.keyframes.clone_from(&action.timeline);
            // Setup action modifiers if there are any
            if let Some(modifiers) = &action.modifiers {
                context.modifier.index = 0;
                context.modifier.instructions = Some(modifiers.clone());
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

            if context.elapsed >= action.total && action.looping {
                context.elapsed = 0;
            }
        }
        None => {
            eprintln!("Action not found");
        }
    }
}

pub fn walk_transition(context: &mut Context, buffer: &InputBuffer) -> bool {
    let input = &buffer.get_curret_input();
    if input.forward {
        context.next = Some(Box::new(standing::WalkForward));
        return true;
    } else if input.backward {
        context.next = Some(Box::new(standing::WalkBackward));
        return true;
    }

    false
}

pub fn dash_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    if buffer.was_motion_executed(Motions::DashForward, buffer.dash) && context.locked.dash_forward
    {
        context.next = Some(Box::new(standing::DashForward));
        return true;
    }
    if buffer.was_motion_executed(Motions::DashBackward, buffer.dash)
        && context.locked.dash_backward
    {
        context.next = Some(Box::new(standing::DashBackward));
        return true;
    }

    if buffer.was_motion_executed(Motions::ForcedDashForward, buffer.forced_dash) {
        context.next = Some(Box::new(standing::DashForward));
        return true;
    }

    if buffer.was_motion_executed(Motions::ForcedDashBackward, buffer.forced_dash) {
        context.next = Some(Box::new(standing::DashBackward));
        return true;
    }

    false
}

// The order of the conditions determines the priority of each attack when pressed simultaneously
pub fn attack_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    if buffer.buffered(&Inputs::HeavyKick, buffer.attack) {
        context.next = Some(Box::new(standing::HeavyKick));
        return true;
    }

    if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) {
        context.next = Some(Box::new(standing::HeavyPunch));
        return true;
    }

    if buffer.buffered(&Inputs::MediumKick, buffer.attack) {
        context.next = Some(Box::new(standing::MediumKick));
        return true;
    }

    if buffer.buffered(&Inputs::MediumPunch, buffer.attack) {
        context.next = Some(Box::new(standing::MediumPunch));
        return true;
    }

    if buffer.buffered(&Inputs::LightKick, buffer.attack) {
        context.next = Some(Box::new(standing::LightKick));
        return true;
    }

    if buffer.buffered(&Inputs::LightPunch, buffer.attack) {
        context.next = Some(Box::new(standing::LightPunch));
        return true;
    }

    false
}

// The order of the conditions determines the priority of each attack when pressed simultaneously
pub fn crouch_attack_transitions(context: &mut Context, buffer: &InputBuffer) -> bool {
    let input = &buffer.get_curret_input();
    if input.down {
        if buffer.buffered(&Inputs::HeavyKick, buffer.attack) {
            context.next = Some(Box::new(crouching::HeavyKick));
            return true;
        }

        if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) {
            context.next = Some(Box::new(crouching::HeavyPunch));
            return true;
        }

        if buffer.buffered(&Inputs::MediumKick, buffer.attack) {
            context.next = Some(Box::new(crouching::MediumKick));
            return true;
        }

        if buffer.buffered(&Inputs::MediumPunch, buffer.attack) {
            context.next = Some(Box::new(crouching::MediumPunch));
            return true;
        }

        if buffer.buffered(&Inputs::LightKick, buffer.attack) {
            context.next = Some(Box::new(crouching::LightKick));
            return true;
        }

        if buffer.buffered(&Inputs::LightPunch, buffer.attack) {
            context.next = Some(Box::new(crouching::LightPunch));
            return true;
        }
    }

    false
}
