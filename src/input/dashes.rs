#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn forward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed(&Motions::DashForward, buffer.dash));
    }

    #[test]
    fn backward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn walking_dash() {
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::Forward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_motion_executed(&Motions::Dp, buffer.dash));

        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::Backward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::Dp, buffer.dash));
    }
}
