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

pub struct GrdStandMidPre;
impl State for GrdStandMidPre {
    fn name(&self) -> String {
        "Rxn GrdStandMidPre".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandMidPre on_enter");
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
        println!("Rxn GrdStandMidPre on_exit");
    }
}

pub struct GrdStandMidEnd;
impl State for GrdStandMidEnd {
    fn name(&self) -> String {
        "Rxn GrdStandMidEnd".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandMidEnd on_enter");
    }

    fn on_update(&mut self, context: &mut Context, buffer: &InputBuffer, physics: &mut Physics) {
        println!("elapsed: {} duration: {}", context.elapsed, context.duration);
        if context.elapsed >= context.duration {
            if jump_transitions(context, buffer, physics) {
                return;
            }
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn GrdStandMidEnd on_exit");
    }
}
