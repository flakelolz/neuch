use crate::prelude::*;

impl InputBuffer {
    pub fn was_dash_executed(&self, motions: Motions, mut time_limit: usize) -> bool {
        if time_limit > (self.buffer.len() + self.index) {
            time_limit = self.buffer.len() + self.index;
        }

        let mut motion_index = 0;

        match motions {
            Motions::DashForward => {
                let motion = [Inputs::Forward, Inputs::Neutral, Inputs::Forward];

                for count in 0..time_limit {
                    let buffer_position = (self.buffer.len() + self.index - (time_limit - 1)
                        + count)
                        % self.buffer.len();

                    let input_command = self.buffer[buffer_position];
                    let direction = motion[motion_index];

                    if Inputs::Down.is_pressed_exclusive(&input_command)
                        || Inputs::Backward.is_pressed_exclusive(&input_command)
                    {
                        motion_index = 0;
                    }

                    if direction.is_pressed(&input_command) {
                        motion_index += 1;
                    }

                    if motion_index >= motion.len() {
                        return true;
                    }
                }
                false
            }
            Motions::DashBackward => {
                let motion = [Inputs::Backward, Inputs::Neutral, Inputs::Backward];

                for count in 0..time_limit {
                    let buffer_position = (self.buffer.len() + self.index - (time_limit - 1)
                        + count)
                        % self.buffer.len();

                    let input_command = self.buffer[buffer_position];
                    let direction = motion[motion_index];

                    if Inputs::Down.is_pressed_exclusive(&input_command)
                        || Inputs::Forward.is_pressed_exclusive(&input_command)
                    {
                        motion_index = 0;
                    }

                    if direction.is_pressed(&input_command) {
                        motion_index += 1;
                    }

                    if motion_index >= motion.len() {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    pub fn validate_dash(&mut self, ctx: &mut SubContext) {
        let duration = 6;
        if self.held(Inputs::Forward, duration) {
            ctx.can_dash_f = false;
        } else if self.held(Inputs::Neutral, duration)
            || self.held(Inputs::Backward, duration)
            || self.held(Inputs::Down, duration)
            || self.held(Inputs::Up, duration)
            || self.was_motion_executed(Motions::ForcedDashForward, self.dash + 5)
        {
            ctx.can_dash_f = true;
        }

        if self.held(Inputs::Backward, duration) {
            ctx.can_dash_b = false;
        } else if self.held(Inputs::Neutral, duration)
            || self.held(Inputs::Forward, duration)
            || self.held(Inputs::Down, duration)
            || self.held(Inputs::Up, duration)
            || self.was_motion_executed(Motions::ForcedDashBackward, self.dash + 5)
        {
            ctx.can_dash_b = true;
        }
    }
}

impl Inputs {
    pub fn is_pressed_exclusive(&self, current: &Input) -> bool {
        match self {
            Inputs::Up => {
                *current
                    == Input {
                        up: true,
                        ..Default::default()
                    }
            }
            Inputs::Down => {
                *current
                    == Input {
                        down: true,
                        ..Default::default()
                    }
            }
            Inputs::Forward => {
                *current
                    == Input {
                        forward: true,
                        ..Default::default()
                    }
            }
            Inputs::Backward => {
                *current
                    == Input {
                        backward: true,
                        ..Default::default()
                    }
            }
            Inputs::UpForward => {
                *current
                    == Input {
                        up: true,
                        forward: true,
                        ..Default::default()
                    }
            }
            Inputs::UpBackward => {
                *current
                    == Input {
                        up: true,
                        backward: true,
                        ..Default::default()
                    }
            }
            Inputs::DownForward => {
                *current
                    == Input {
                        down: true,
                        forward: true,
                        ..Default::default()
                    }
            }
            Inputs::DownBackward => {
                *current
                    == Input {
                        down: true,
                        backward: true,
                        ..Default::default()
                    }
            }
            Inputs::Neutral => {
                *current
                    == Input {
                        up: false,
                        down: false,
                        forward: false,
                        backward: false,
                        ..Default::default()
                    }
            }
            Inputs::LightPunch => {
                *current
                    == Input {
                        lp: true,
                        ..Default::default()
                    }
            }
            Inputs::MediumPunch => {
                *current
                    == Input {
                        mp: true,
                        ..Default::default()
                    }
            }
            Inputs::HeavyPunch => {
                *current
                    == Input {
                        hp: true,
                        ..Default::default()
                    }
            }
            Inputs::LightKick => {
                *current
                    == Input {
                        lk: true,
                        ..Default::default()
                    }
            }
            Inputs::MediumKick => {
                *current
                    == Input {
                        mk: true,
                        ..Default::default()
                    }
            }
            Inputs::HeavyKick => {
                *current
                    == Input {
                        hk: true,
                        ..Default::default()
                    }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn is_pressed_exclusive() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        let current = buffer.current();
        assert!(Inputs::Forward.is_pressed_exclusive(current));

        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownForward);
        let current = buffer.current();
        assert!(!Inputs::Forward.is_pressed_exclusive(current));
    }

    #[test]
    fn forward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_dash_executed(Motions::DashForward, buffer.dash));
    }

    #[test]
    fn backward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_4_5_6_5_4_repeating_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_dash_executed(Motions::DashForward, buffer.dash));

        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_4_6_5_6_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_dash_executed(Motions::DashForward, buffer.dash));
    }

    #[test]
    fn dash_with_4_held_5_4_should_fail() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        assert!(!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash) && ctx.can_dash_b));
    }

    #[test]
    fn dash_with_4_held_5_4_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=10 {
            dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        assert!((buffer.was_dash_executed(Motions::DashBackward, buffer.dash) && ctx.can_dash_b));
    }

    #[test]
    fn dash_with_1_held_4_5_4_should_fail() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            // This prevents the empty neutral spaces to force a dash
            dash_helper(&mut buffer, Inputs::Down, &mut ctx);
        }
        for _ in 20..=27 {
            dash_helper(&mut buffer, Inputs::DownBackward, &mut ctx);
        }
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        assert!(!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash) && ctx.can_dash_b));
    }

    #[test]
    fn dash_with_1_held_5_4_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            // This prevents the empty neutral spaces to force a dash
            dash_helper(&mut buffer, Inputs::Down, &mut ctx);
        }
        for _ in 20..=27 {
            dash_helper(&mut buffer, Inputs::DownBackward, &mut ctx);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx);
        assert!((buffer.was_dash_executed(Motions::DashBackward, buffer.dash) && ctx.can_dash_b));
    }

    #[test]
    fn dash_with_1_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_4_5_1_4_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash));
    }

    #[test]
    fn dash_with_2_in_the_middle_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash));
    }
}
