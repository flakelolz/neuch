use crate::prelude::*;

pub struct HitStandMid;

impl State for HitStandMid {
    fn name(&self) -> String {
        "Rxn HitStandMid".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        println!("Rxn HitStandMid on_enter");
    }

    fn on_update(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        if context.elapsed >= context.duration {
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, context: &mut Context, _buffer: &InputBuffer, _physics: &mut Physics) {
        context.reaction.reset_def();
        println!("Rxn HitStandMid on_exit");
    }
}
