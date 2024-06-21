use crate::{prelude::*, state_machine::transitions::attack_transitions};

pub struct Idle;
impl State for Idle {
    fn name(&self) -> String {
        String::from("St Idle")
    }
    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St Idle on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, physics: &mut Physics) {
        physics.velocity.x = 0;

        attack_transitions(context, input);

        if input.forward {
            context.next = Some(Box::new(super::WalkForward));
        }

        if input.backward {
            context.next = Some(Box::new(super::WalkBackward));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St Idle on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        String::from("St LightPunch")
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightPunch on_enter");
    }

    fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightPunch on_update");
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("St LightPunch on_exit");
    }
}

// pub struct MediumPunch;
// impl State for MediumPunch {
//     fn name(&self) -> &'static str {
//         "St MediumPunch"
//     }
//
//     fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St MediumPunch on_enter");
//     }
//
//     fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St MediumPunch on_update");
//     }
//
//     fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St MediumPunch on_exit");
//     }
// }
//
// pub struct HeavyPunch;
// impl State for HeavyPunch {
//     fn name(&self) -> &'static str {
//         "HeavyPunch"
//     }
//
//     fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyPunch on_enter");
//     }
//
//     fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyPunch on_update");
//     }
//
//     fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyPunch on_exit");
//     }
// }
//
// pub struct LightKick;
// impl State for LightKick {
//     fn name(&self) -> &'static str {
//         "St LightKick"
//     }
//
//     fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St LightKick on_enter");
//     }
//
//     fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St LightKick on_update");
//     }
//
//     fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St LightKick on_exit");
//     }
// }
//
// pub struct MediumKick;
// impl State for MediumKick {
//     fn name(&self) -> &'static str {
//         "MediumKick"
//     }
//
//     fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("MediumKick on_enter");
//     }
//
//     fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("MediumKick on_update");
//     }
//
//     fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("MediumKick on_exit");
//     }
// }
//
// pub struct HeavyKick;
// impl State for HeavyKick {
//     fn name(&self) -> &'static str {
//         "St HeavyKick"
//     }
//
//     fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyKick on_enter");
//     }
//
//     fn on_update(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyKick on_update");
//     }
//
//     fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
//         println!("St HeavyKick on_exit");
//     }
// }
