use crate::prelude::*;

pub struct Start;
impl State for Start {
    fn name(&self) -> String {
        "Cr Start".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr Start on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        crouch_attack_transitions(context, input);

        if context.elapsed >= context.duration - 1 && input.down {
            context.next = Some(Box::new(Idle));
        }

        if !input.down {
            if input.forward {
                context.next = Some(Box::new(standing::WalkForward));
                return;
            }

            if input.backward {
                context.next = Some(Box::new(standing::WalkBackward));
                return;
            }

            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr Start on_exit");
    }
}

pub struct Idle;
impl State for Idle {
    fn name(&self) -> String {
        "Cr Idle".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr Idle on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        crouch_attack_transitions(context, input);

        if input.forward && !input.down {
            context.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if input.backward && !input.down {
            context.next = Some(Box::new(standing::WalkBackward));
            return;
        }

        if !input.down {
            context.next = Some(Box::new(crouching::End));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr Idle on_exit");
    }
}

pub struct End;
impl State for End {
    fn name(&self) -> String {
        "Cr End".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr End on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        attack_transitions(context, input);

        if input.forward {
            context.next = Some(Box::new(standing::WalkForward));
            return;
        }

        if input.backward {
            context.next = Some(Box::new(standing::WalkBackward));
            return;
        }

        if input.down {
            crouch_attack_transitions(context, input);
        }

        if context.elapsed >= context.duration - 1 {
            context.next = Some(Box::new(standing::Idle));
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr End on_exit");
    }
}

pub struct LightPunch;
impl State for LightPunch {
    fn name(&self) -> String {
        "Cr LightPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr LightPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        handle_chainable(context, input);

        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr LightPunch on_exit");
    }
}

pub struct MediumPunch;
impl State for MediumPunch {
    fn name(&self) -> String {
        "Cr MediumPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr MediumPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr MediumPunch on_exit");
    }
}

pub struct HeavyPunch;
impl State for HeavyPunch {
    fn name(&self) -> String {
        "Cr HeavyPunch".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr HeavyPunch on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr HeavyPunch on_exit");
    }
}

pub struct LightKick;
impl State for LightKick {
    fn name(&self) -> String {
        "Cr LightKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr LightKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        handle_chainable(context, input);

        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr LightKick on_exit");
    }
}

pub struct MediumKick;
impl State for MediumKick {
    fn name(&self) -> String {
        "Cr MediumKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr MediumKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr MediumKick on_exit");
    }
}

pub struct HeavyKick;
impl State for HeavyKick {
    fn name(&self) -> String {
        "Cr HeavyKick".to_owned()
    }

    fn on_enter(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr HeavyKick on_enter");
    }

    fn on_update(&mut self, context: &mut Context, input: &Input, _physics: &mut Physics) {
        if context.elapsed >= context.duration - 1 {
            if input.down {
                if crouch_attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(crouching::Idle));
            }

            if !input.down {
                if attack_transitions(context, input) {
                    return;
                }

                context.next = Some(Box::new(standing::Idle));
            }
        }
    }

    fn on_exit(&mut self, _context: &mut Context, _input: &Input, _physics: &mut Physics) {
        println!("Cr HeavyKick on_exit");
    }
}
