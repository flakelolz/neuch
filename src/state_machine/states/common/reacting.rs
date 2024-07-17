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
        println!("hit: {}", context.reaction.hitstun);
        if context.elapsed >= context.duration {
            if jump_transitions(context, buffer, physics) {
                return;
            }
            context.ctx.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _buffer: &InputBuffer, physics: &mut Physics) {
        physics.velocity.x = 0;
        println!("Rxn HitStandMid on_exit");
    }
}
