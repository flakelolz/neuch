use crate::prelude::*;

pub struct UpperWeak;
impl State for UpperWeak {
    fn name(&self) -> String {
        "Rxn UpperWeak".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperWeak on_enter");
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
        println!("Rxn UpperWeak on_exit");
    }
}

pub struct UpperMid;
impl State for UpperMid {
    fn name(&self) -> String {
        "Rxn UpperMid".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperMid on_enter");
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
        println!("Rxn UpperMid on_exit");
    }
}

pub struct UpperStrong;
impl State for UpperStrong {
    fn name(&self) -> String {
        "Rxn UpperStrong".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperStrong on_enter");
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
        println!("Rxn UpperStrong on_exit");
    }
}

pub struct LowerWeak;
impl State for LowerWeak {
    fn name(&self) -> String {
        "Rxn LowerWeak".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerWeak on_enter");
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
        println!("Rxn LowerWeak on_exit");
    }
}

pub struct LowerMid;
impl State for LowerMid {
    fn name(&self) -> String {
        "Rxn LowerMid".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerMid on_enter");
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
        println!("Rxn LowerMid on_exit");
    }
}

pub struct LowerStrong;
impl State for LowerStrong {
    fn name(&self) -> String {
        "Rxn LowerStrong".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerStrong on_enter");
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
        println!("Rxn LowerStrong on_exit");
    }
}

pub struct CrouchWeak;
impl State for CrouchWeak {
    fn name(&self) -> String {
        "Rxn CrouchWeak".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn CrouchWeak on_enter");
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
        println!("Rxn CrouchWeak on_exit");
    }
}


pub struct CrouchMid;
impl State for CrouchMid {
    fn name(&self) -> String {
        "Rxn CrouchMid".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn CrouchMid on_enter");
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
        println!("Rxn CrouchMid on_exit");
    }
}

pub struct CrouchStrong;
impl State for CrouchStrong {
    fn name(&self) -> String {
        "Rxn CrouchStrong".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn CrouchStrong on_enter");
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
        println!("Rxn CrouchStrong on_exit");
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
