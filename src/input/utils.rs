use crate::prelude::*;

pub fn neutral(buffer: &InputBuffer) -> bool {
    !buffer.current().up
        && !buffer.current().down
        && !buffer.current().forward
        && !buffer.current().backward
}

pub fn up(buffer: &InputBuffer) -> bool {
    buffer.current().up
}

pub fn down(buffer: &InputBuffer) -> bool {
    buffer.current().down
}

pub fn backward(buffer: &InputBuffer) -> bool {
    buffer.current().backward
}

pub fn forward(buffer: &InputBuffer) -> bool {
    buffer.current().forward
}

pub fn up_forward(buffer: &InputBuffer) -> bool {
    up(buffer) && forward(buffer)
}

pub fn up_backward(buffer: &InputBuffer) -> bool {
    up(buffer) && backward(buffer)
}

/// Checks if there is a direction that would invalidate the whole motion input
pub fn check_invalid_motion(motions: Motions, buffer: &InputBuffer, duration: usize) -> bool {
    match motions {
        Motions::DashForward => {
            buffer.buffered(Inputs::Backward, duration) || buffer.buffered(Inputs::Down, duration)
        }
        Motions::DashBackward => {
            buffer.buffered(Inputs::Forward, duration) || buffer.buffered(Inputs::Down, duration)
        }
        _ => false,
    }
}

pub fn test_helper(buffer: &mut InputBuffer, inputs: Inputs) {
    match inputs {
        Inputs::Up => {
            buffer.update(&Input {
                up: true,
                ..Default::default()
            });
        }
        Inputs::Down => {
            buffer.update(&Input {
                down: true,
                ..Default::default()
            });
        }
        Inputs::Forward => {
            buffer.update(&Input {
                forward: true,
                ..Default::default()
            });
        }
        Inputs::Backward => {
            buffer.update(&Input {
                backward: true,
                ..Default::default()
            });
        }
        Inputs::UpForward => {
            buffer.update(&Input {
                up: true,
                forward: true,
                ..Default::default()
            });
        }
        Inputs::UpBackward => {
            buffer.update(&Input {
                up: true,
                backward: true,
                ..Default::default()
            });
        }
        Inputs::DownForward => {
            buffer.update(&Input {
                down: true,
                forward: true,
                ..Default::default()
            });
        }
        Inputs::DownBackward => {
            buffer.update(&Input {
                down: true,
                backward: true,
                ..Default::default()
            });
        }
        Inputs::Neutral => {
            buffer.update(&Input::default());
        }
        Inputs::LightPunch => {
            buffer.update(&Input {
                lp: true,
                ..Default::default()
            });
        }
        Inputs::MediumPunch => {
            buffer.update(&Input {
                mp: true,
                ..Default::default()
            });
        }
        Inputs::HeavyPunch => {
            buffer.update(&Input {
                hp: true,
                ..Default::default()
            });
        }
        Inputs::LightKick => {
            buffer.update(&Input {
                lk: true,
                ..Default::default()
            });
        }
        Inputs::MediumKick => {
            buffer.update(&Input {
                mk: true,
                ..Default::default()
            });
        }
        Inputs::HeavyKick => {
            buffer.update(&Input {
                hk: true,
                ..Default::default()
            });
        }
    }
}

pub fn dash_helper(buffer: &mut InputBuffer, inputs: Inputs, ctx: &mut SubContext) {
    test_helper(buffer, inputs);
    buffer.validate_dash(ctx);
}
