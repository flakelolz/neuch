use crate::prelude::*;

pub fn neutral(buffer: &InputBuffer) -> bool {
    !buffer.current().up
        && !buffer.current().down
        && !buffer.current().right
        && !buffer.current().left
}

pub fn up(buffer: &InputBuffer) -> bool {
    buffer.current().up
}

pub fn down(buffer: &InputBuffer) -> bool {
    buffer.current().down
}

pub fn backward(buffer: &InputBuffer, flipped: &bool) -> bool {
    if *flipped {
        buffer.current().right
    } else {
        buffer.current().left
    }
}

pub fn forward(buffer: &InputBuffer, flipped: &bool) -> bool {
    if *flipped {
        buffer.current().left
    } else {
        buffer.current().right
    }
}

pub fn up_forward(buffer: &InputBuffer, flipped: &bool) -> bool {
    up(buffer) && forward(buffer, flipped)
}

pub fn up_backward(buffer: &InputBuffer, flipped: &bool) -> bool {
    up(buffer) && backward(buffer, flipped)
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
                right: true,
                ..Default::default()
            });
        }
        Inputs::Backward => {
            buffer.update(&Input {
                left: true,
                ..Default::default()
            });
        }
        Inputs::UpForward => {
            buffer.update(&Input {
                up: true,
                right: true,
                ..Default::default()
            });
        }
        Inputs::UpBackward => {
            buffer.update(&Input {
                up: true,
                left: true,
                ..Default::default()
            });
        }
        Inputs::DownForward => {
            buffer.update(&Input {
                down: true,
                right: true,
                ..Default::default()
            });
        }
        Inputs::DownBackward => {
            buffer.update(&Input {
                down: true,
                left: true,
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

pub fn dash_helper(buffer: &mut InputBuffer, inputs: Inputs, ctx: &mut SubContext, flipped: &bool) {
    test_helper(buffer, inputs);
    buffer.lockout_dash(ctx, flipped, 6);
}
