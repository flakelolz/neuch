use crate::prelude::*;

use super::*;

pub fn handle_transition(
    processor: &mut StateProcessor,
    context: &mut Context,
    input: &Input,
    physics: &mut Physics,
    character: &Character,
    animator: &mut Animator,
) {
    // If there is a next state to transition to it
    if let Some(mut next) = context.next.take() {
        context.elapsed = 0;
        processor.current.on_exit(context, input, physics);
        next.on_enter(context, input, physics);
        processor.current = next;
        animator.reset();

        // Set animnation data
        let name = processor.current.name();
        if let Some(action) = find_action(character, &name) {
            context.duration = action.total;
            animator.keyframes.clone_from(&action.timeline);
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
            println!("Action not found");
        }
    }
}

pub fn attack_transitions(context: &mut Context, input: &Input) -> bool {
    if input.lp {
        context.next = Some(Box::new(standing::LightPunch));
        return true;
    }

    if input.lk {
        context.next = Some(Box::new(standing::LightKick));
        return true;
    }

    if input.mp {
        context.next = Some(Box::new(standing::MediumPunch));
        return true;
    }

    if input.mk {
        context.next = Some(Box::new(standing::MediumKick));
        return true;
    }

    if input.hp {
        context.next = Some(Box::new(standing::HeavyPunch));
        return true;
    }

    if input.hk {
        context.next = Some(Box::new(standing::HeavyKick));
        return true;
    }

    false
}
