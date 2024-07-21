use crate::prelude::*;

pub struct HitStandMid;
impl State for HitStandMid {
    fn name(&self) -> String {
        "Rxn HitStandMid".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn HitStandMid on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed >= context.duration {
            if jump_transitions(context, buffer, physics) {
                return;
            }
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn HitStandMid on_exit");
    }
}

pub struct GrdStandPre;
impl State for GrdStandPre {
    fn name(&self) -> String {
        "Rxn GrdStandPre".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandPre on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed >= context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandPre on_exit");
    }
}

pub struct GrdStandEnd;
impl State for GrdStandEnd {
    fn name(&self) -> String {
        "Rxn GrdStandEnd".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed >= context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_exit");
    }
}

pub struct GrdCrouchPre;
impl State for GrdCrouchPre {
    fn name(&self) -> String {
        "Rxn GrdCrouchPre".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchPre on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed >= context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchPre on_exit");
    }
}

pub struct GrdCrouchEnd;
impl State for GrdCrouchEnd {
    fn name(&self) -> String {
        "Rxn GrdCrouchEnd".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        if context.elapsed >= context.duration {
            // Transitions
            if turn_transition(&mut context.ctx, buffer, physics) {
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

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_exit");
    }
}
