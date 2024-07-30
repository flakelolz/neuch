#[cfg(test)]
mod tests {
    use super::super::*;

    // NOTE: Motion testing

    #[test]
    fn was_motion_executed() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::DownForward);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed_in_time(Motions::Qcf, 3));
    }

    #[test]
    fn half_circle_forward_full() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::DownForward);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed_in_time(Motions::Hcf, 5));
    }

    #[test]
    fn half_circle_forward_no_down() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::DownForward);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed_in_time(Motions::Hcf, 4));
    }

    #[test]
    fn half_circle_forward_cardinals() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_motion_executed_in_time(Motions::Hcf, 3));
    }

    #[test]
    fn half_circle_forward_fails() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::DownForward);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_motion_executed_in_time(Motions::Hcf, 4));
    }
}
