use super::*;

pub struct Standing;
impl State for Standing {
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Standing on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        println!("Standing on_update");

        physics.velocity.x = 0;

        if input.forward {
            context.next = Some(Box::new(WalkForward));
        }

        if input.backward {
            context.next = Some(Box::new(WalkBackward));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Standin on_exit");
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        println!("WalkForward on_update");

        physics.velocity.x = 3000;

        if !input.forward {
            context.next = Some(Box::new(Standing));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        println!("WalkBackward on_update");

        physics.velocity.x = -3000;

        if !input.backward {
            context.next = Some(Box::new(Standing));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("WalkBackward on_exit");
    }
}
