pub mod standing;
use crate::prelude::*;

pub struct WalkForward;
impl State for WalkForward {
    fn name(&self) -> &'static str {
        "WalkForward"
    }
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = 3000;

        if !input.forward {
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn name(&self) -> &'static str {
        "WalkBackward"
    }
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = -3000;

        if !input.backward {
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkBackward on_exit");
    }
}
