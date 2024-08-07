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
        common_standing_reaction_transitions(context, buffer, physics);
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
        common_standing_reaction_transitions(context, buffer, physics);
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
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperStrong on_exit");
    }
}

pub struct UpperRising;
impl State for UpperRising {
    fn name(&self) -> String {
        "Rxn UpperRising".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperRising on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn UpperRising on_exit");
    }
}

pub struct FrontSpin;
impl State for FrontSpin {
    fn name(&self) -> String {
        "Rxn FrontSpin".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn FrontSpin on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn FrontSpin on_exit");
    }
}

pub struct BackSpin;
impl State for BackSpin {
    fn name(&self) -> String {
        "Rxn BackSpin".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn BackSpin on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn BackSpin on_exit");
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
        common_standing_reaction_transitions(context, buffer, physics);
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
        common_standing_reaction_transitions(context, buffer, physics);
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
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerStrong on_exit");
    }
}

pub struct LowerRising;
impl State for LowerRising {
    fn name(&self) -> String {
        "Rxn LowerRising".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerRising on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn LowerRising on_exit");
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
        common_crouching_reaction_transitions(context, buffer, physics);
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
        common_crouching_reaction_transitions(context, buffer, physics);
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
        common_crouching_reaction_transitions(context, buffer, physics);
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
        common_standing_reaction_transitions(context, buffer, physics);
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

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_enter");
        context.ctx.reaction.blocking = true;
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_standing_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandEnd on_exit");
        context.ctx.reaction.blocking = false;
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
        common_crouching_reaction_transitions(context, buffer, physics);
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

    fn on_enter(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_enter");
        context.ctx.reaction.blocking = true;
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        common_crouching_reaction_transitions(context, buffer, physics);
    }

    fn on_exit(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdCrouchEnd on_exit");
        context.ctx.reaction.blocking = false;
    }
}
