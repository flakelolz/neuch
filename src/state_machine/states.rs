use super::{Context, State};

pub struct Standing;
impl State for Standing {
    fn on_enter(&mut self, _context: &mut Context) {
        println!("Standing on_enter");
    }

    fn on_update(&mut self, context: &mut Context) {
        println!("Standing on_update");

        context.physics.velocity.x = 0;

        if context.input.forward {
            context.next = Some(Box::new(WalkForward));
        }

        if context.input.backward {
            context.next = Some(Box::new(WalkBackward));
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        println!("Standin on_exit");
    }
}

pub struct WalkForward;
impl State for WalkForward {
    fn on_enter(&mut self, _context: &mut Context) {
        println!("WalkForward on_enter");
    }

    fn on_update(&mut self, context: &mut Context) {
        println!("WalkForward on_update");

        context.physics.velocity.x = 3000;

        if !context.input.forward {
            context.next = Some(Box::new(Standing));
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        println!("WalkForward on_exit");
    }
}

pub struct WalkBackward;
impl State for WalkBackward {
    fn on_enter(&mut self, _context: &mut Context) {
        println!("WalkBackward on_enter");
    }

    fn on_update(&mut self, context: &mut Context) {
        println!("WalkBackward on_update");

        context.physics.velocity.x = -3000;

        if !context.input.backward {
            context.next = Some(Box::new(Standing));
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        println!("WalkBackward on_exit");
    }
}
