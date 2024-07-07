use crate::prelude::*;

pub fn neutral(buffer: &InputBuffer) -> bool {
    !buffer.input().up
        && !buffer.input().down
        && !buffer.input().forward
        && !buffer.input().backward
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
