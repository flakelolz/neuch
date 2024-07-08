use crate::prelude::*;

pub fn neutral(buffer: &InputBuffer) -> bool {
    !buffer.input().up
        && !buffer.input().down
        && !buffer.input().forward
        && !buffer.input().backward
}

pub fn up(buffer: &InputBuffer) -> bool {
    buffer.input().up
}

pub fn down(buffer: &InputBuffer) -> bool {
    buffer.input().down
}

pub fn backward(buffer: &InputBuffer) -> bool {
    buffer.input().backward
}

pub fn forward(buffer: &InputBuffer) -> bool {
    buffer.input().forward
}

pub fn up_forward(buffer: &InputBuffer) -> bool {
    up(buffer) && forward(buffer)
}

pub fn up_backward(buffer: &InputBuffer) -> bool {
    up(buffer) && backward(buffer)
}

/// Checks if there is a direction that would invalidate the whole motion input
pub fn check_invalid_motion(motions: &Motions, buffer: &InputBuffer, duration: usize) -> bool {
    match motions {
        Motions::DashForward => {
            buffer.buffered(&Inputs::Backward, duration) || buffer.buffered(&Inputs::Down, duration)
        }
        Motions::DashBackward => {
            buffer.buffered(&Inputs::Forward, duration) || buffer.buffered(&Inputs::Down, duration)
        }
        Motions::Dp => {
            buffer.buffered(&Inputs::Backward, duration)
                || buffer.buffered(&Inputs::DownBackward, duration)
        }
        Motions::RDp => {
            buffer.buffered(&Inputs::Forward, duration)
                || buffer.buffered(&Inputs::DownForward, duration)
        }
        _ => false,
    }
}
