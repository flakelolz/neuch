#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn was_pressed() {
        let mut buffer = InputBuffer::default();
        let input = Input::default();

        buffer.update(&input);

        let input = Input {
            down: true,
            ..Default::default()
        };

        buffer.update(&input);

        let current = buffer.get_curret_input();
        let previous = buffer.get_previous_input();

        let inputs = Inputs::Down;

        assert!(inputs.was_initially_pressed(&current, &previous));
    }

    #[test]
    fn was_input_pressed() {
        let mut buffer = InputBuffer::default();

        for _ in 0..1 {
            let input = Input {
                forward: true,
                ..Default::default()
            };

            buffer.update(&input);
        }

        for _ in 1..2 {
            let input = Input::default();
            buffer.update(&input);
        }

        for _ in 2..3 {
            let input = Input {
                forward: true,
                ..Default::default()
            };

            buffer.update(&input);
        }

        assert!(buffer.was_input_pressed(&Inputs::Forward));
    }

    #[test]
    fn was_input_pressed_on_frame() {
        let mut buffer = InputBuffer::default();
        let input = Input {
            up: true,
            ..Default::default()
        };

        buffer.update(&input);

        let input = Input {
            down: true,
            up: true,
            ..Default::default()
        };

        buffer.update(&input);

        assert!(buffer.was_input_pressed_on_frame(&Inputs::Down, 1));
        assert!(!buffer.was_input_pressed_on_frame(&Inputs::Up, 1));
    }

    #[test]
    fn was_input_pressed_buffered() {
        let mut buffer = InputBuffer::default();

        for _ in 0..3 {
            let input = Input {
                down: true,
                ..Default::default()
            };
            buffer.update(&input);
        }

        {
            let input = Input {
                up: true,
                ..Default::default()
            };
            buffer.update(&input);
        }

        for _ in 4..6 {
            let input = Input::default();
            buffer.update(&input);
        }

        assert!(buffer.was_input_pressed_buffered(&Inputs::Up, 2));
        assert!(buffer.was_input_pressed_buffered(&Inputs::Down, 5));
    }

    #[test]
    fn was_input_held() {
        let mut buffer = InputBuffer::default();
        for _ in 0..5 {
            let input = Input {
                down: true,
                ..Default::default()
            };
            buffer.update(&input);
        }

        assert!(buffer.is_input_held(&Inputs::Down, 4));
    }

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

        assert!(buffer.was_motion_executed(Motions::Qcf, 3));
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

        assert!(buffer.was_motion_executed(Motions::Hcf, 5));
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

        assert!(buffer.was_motion_executed(Motions::Hcf, 4));
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

        assert!(buffer.was_motion_executed(Motions::Hcf, 3));
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

        assert!(!buffer.was_motion_executed(Motions::Hcf, 4));
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

        assert!(buffer.was_motion_executed(Motions::DashForward, 3));
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

        assert!(buffer.was_motion_executed(Motions::DashBackward, 3));
    }
}
