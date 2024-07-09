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
    fn dash_with_4_5_6_5_4_repeating_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_motion_executed(&Motions::DashForward, buffer.dash));

        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_4_held_5_4_should_fail() {
        // Pressed for 5 frames is fine
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::Backward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
        // Pressed for 6 frames is too long
        let mut buffer = InputBuffer::default();
        for _ in 0..6 {
            test_helper(&mut buffer, Inputs::Forward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_motion_executed(&Motions::Dp, buffer.dash));
    }

    #[test]
    fn dash_with_4_held_5_4_5_4_should_work() {
        // Pressed for 5 frames is fine
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::Forward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed(&Motions::DashForward, buffer.dash));
        // Pressed for 6 frames is too long
        let mut buffer = InputBuffer::default();
        for _ in 0..6 {
            test_helper(&mut buffer, Inputs::Backward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_1_held_4_5_4_should_fail() {
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::DownBackward);
        }
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_1_held_5_4_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            test_helper(&mut buffer, Inputs::DownBackward);
        }
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_1_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_5_5_1_4_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_2_in_the_middle_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_motion_executed(&Motions::DashBackward, buffer.dash));
    }
}
