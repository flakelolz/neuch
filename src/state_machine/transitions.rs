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
    // state name to find the action. Needs both ActionData and ActionMap
    // Advance the timeline of the animation until it reaches the state duration
    // Then either loop if looping is true or change state if looping is false
    // based on the current state's transition conditions
    context.elapsed += 1;

    let name = processor.current.name();
    let action = find_action(character, &name);

    match action {
        Some(action) => {
            context.duration = action.total;
            animator.total = context.duration;
            animator.current = context.elapsed;
            animator.keyframes.clone_from(&action.timeline);

            if context.elapsed >= action.total && action.looping {
                context.elapsed = 0;
            }
        }
        None => {
            println!("Action not found");
        }
    }

    if let Some(mut next) = context.next.take() {
        context.elapsed = 0;
        processor.current.on_exit(context, input, physics);
        next.on_enter(context, input, physics);
        processor.current = next;
        animator.reset();
    }
}

pub fn attack_transitions(context: &mut Context, input: &Input) -> bool {
    if input.lp {
        context.next = Some(Box::new(standing::LightPunch));
        return true;
    }

    // if input.lk {
    //     context.next = Some(Box::new(standing::LightKick));
    //     return true;
    // }
    //
    // if input.mp {
    //     context.next = Some(Box::new(standing::MediumPunch));
    //     return true;
    // }
    //
    // if input.mk {
    //     context.next = Some(Box::new(standing::MediumKick));
    //     return true;
    // }
    //
    // if input.hp {
    //     context.next = Some(Box::new(standing::HeavyPunch));
    //     return true;
    // }
    //
    // if input.hk {
    //     context.next = Some(Box::new(standing::HeavyKick));
    //     return true;
    // }

    false
}
