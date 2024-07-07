#[cfg(test)]
mod tests {
    use super::super::*;

    // NOTE: Motion testing
    // FIX: Learn Rust macros really well to make this cleaner

    #[test]
    fn was_motion_executed() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            down: true,
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::Qcf, 3));
    }

    #[test]
    fn half_circle_forward_full() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            backward: true,
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::Hcf, 5));
    }

    #[test]
    fn half_circle_forward_no_down() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            backward: true,
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::Hcf, 4));
    }

    #[test]
    fn half_circle_forward_cardinals() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::Hcf, 3));
    }

    #[test]
    fn half_circle_forward_fails() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            down: true,
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            down: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            down: true,
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(!buffer.was_motion_executed(&Motions::Hcf, 4));
    }

    #[test]
    fn forward_dash() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            forward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::DashForward, 3));
    }

    #[test]
    fn backward_dash() {
        let mut buffer = InputBuffer::default();

        let input = Input {
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            ..Default::default()
        };
        buffer.update(&input);

        let input = Input {
            backward: true,
            ..Default::default()
        };
        buffer.update(&input);

        assert!(buffer.was_motion_executed(&Motions::DashBackward, 3));
    }
}
