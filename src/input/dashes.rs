use crate::prelude::*;

impl InputBuffer {
    pub fn was_dash_executed(
        &self,
        motions: Motions,
        mut time_limit: usize,
        flipped: &bool,
    ) -> bool {
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

                    // Invalidate if there's a down or backward input
                    if Inputs::Down.is_pressed_exclusive(&input_command, flipped)
                        || Inputs::Backward.is_pressed_exclusive(&input_command, flipped)
                    {
                        motion_index = 0;
                    }

                    // Invalidate if there's a down forward input before the last forward
                    if motion_index == 2
                        && Inputs::DownForward.is_pressed_exclusive(&input_command, flipped)
                    {
                        motion_index = 0;
                    }

                    if direction.is_pressed(&input_command, flipped) {
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

                    // Invalidate if there's a down or forward input
                    if Inputs::Down.is_pressed_exclusive(&input_command, flipped)
                        || Inputs::Forward.is_pressed_exclusive(&input_command, flipped)
                    {
                        motion_index = 0;
                    }

                    // Invalidate if there's a down back input before the last forward
                    if motion_index == 2
                        && Inputs::DownBackward.is_pressed_exclusive(&input_command, flipped)
                    {
                        motion_index = 0;
                    }

                    if direction.is_pressed(&input_command, flipped) {
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

    pub fn validate_dash(&mut self, ctx: &mut SubContext, flipped: &bool) {
        let duration = 6;
        if self.held(Inputs::Forward, duration, flipped) {
            ctx.can_dash_f = false;
        } else if self.held(Inputs::Neutral, duration, flipped)
            || self.held(Inputs::Backward, duration, flipped)
            || self.held(Inputs::Down, duration, flipped)
            || self.held(Inputs::Up, duration, flipped)
            || self.was_motion_executed(Motions::ForcedDashForward, self.dash + 5, flipped)
        {
            ctx.can_dash_f = true;
        }

        if self.held(Inputs::Backward, duration, flipped) {
            ctx.can_dash_b = false;
        } else if self.held(Inputs::Neutral, duration, flipped)
            || self.held(Inputs::Forward, duration, flipped)
            || self.held(Inputs::Down, duration, flipped)
            || self.held(Inputs::Up, duration, flipped)
            || self.was_motion_executed(Motions::ForcedDashBackward, self.dash + 5, flipped)
        {
            ctx.can_dash_b = true;
        }
    }
}

impl Inputs {
    pub fn is_pressed_exclusive(&self, current: &Input, flipped: &bool) -> bool {
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
                if *flipped {
                    *current
                        == Input {
                            left: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            right: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::Backward => {
                if *flipped {
                    *current
                        == Input {
                            right: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            left: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::UpForward => {
                if *flipped {
                    *current
                        == Input {
                            up: true,
                            left: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            up: true,
                            right: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::UpBackward => {
                if *flipped {
                    *current
                        == Input {
                            up: true,
                            right: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            up: true,
                            left: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::DownForward => {
                if *flipped {
                    *current
                        == Input {
                            down: true,
                            left: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            down: true,
                            right: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::DownBackward => {
                if *flipped {
                    *current
                        == Input {
                            down: true,
                            right: true,
                            ..Default::default()
                        }
                } else {
                    *current
                        == Input {
                            down: true,
                            left: true,
                            ..Default::default()
                        }
                }
            }
            Inputs::Neutral => {
                *current
                    == Input {
                        up: false,
                        down: false,
                        right: false,
                        left: false,
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

    pub fn is_pressed_exclusive_dir(&self, current: &Input, flipped: &bool) -> bool {
        match self {
            Inputs::Up => matches!(
                current,
                Input {
                    up: true,
                    down: false,
                    left: false,
                    right: false,
                    ..
                }
            ),
            Inputs::Down => matches!(
                current,
                Input {
                    down: true,
                    up: false,
                    left: false,
                    right: false,
                    ..
                }
            ),
            Inputs::Forward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            left: true,
                            up: false,
                            down: false,
                            right: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            right: true,
                            up: false,
                            down: false,
                            left: false,
                            ..
                        }
                    )
                }
            }
            Inputs::Backward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            right: true,
                            left: false,
                            up: false,
                            down: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            left: true,
                            right: false,
                            up: false,
                            down: false,
                            ..
                        }
                    )
                }
            }
            Inputs::UpForward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            up: true,
                            left: true,
                            right: false,
                            down: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            up: true,
                            right: true,
                            left: false,
                            down: false,
                            ..
                        }
                    )
                }
            }
            Inputs::UpBackward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            up: true,
                            right: true,
                            left: false,
                            down: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            up: true,
                            left: true,
                            right: false,
                            down: false,
                            ..
                        }
                    )
                }
            }
            Inputs::DownForward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            down: true,
                            left: true,
                            right: false,
                            up: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            down: true,
                            right: true,
                            left: false,
                            up: false,
                            ..
                        }
                    )
                }
            }
            Inputs::DownBackward => {
                if *flipped {
                    matches!(
                        current,
                        Input {
                            down: true,
                            right: true,
                            left: false,
                            up: false,
                            ..
                        }
                    )
                } else {
                    matches!(
                        current,
                        Input {
                            down: true,
                            left: true,
                            right: false,
                            up: false,
                            ..
                        }
                    )
                }
            }
            Inputs::Neutral => matches!(
                current,
                Input {
                    down: false,
                    up: false,
                    left: false,
                    right: false,
                    ..
                }
            ),
            _ => panic!("Only for directions"),
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
        assert!(Inputs::Forward.is_pressed_exclusive(current, &false));

        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownForward);
        let current = buffer.current();
        assert!(!Inputs::Forward.is_pressed_exclusive(current, &false));
    }

    #[test]
    fn forward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_dash_executed(Motions::DashForward, buffer.dash, &false));
    }

    #[test]
    fn backward_dash() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_4_5_6_5_4_repeating_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(!buffer.was_dash_executed(Motions::DashForward, buffer.dash, &false));

        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_4_6_5_6_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Forward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Forward);
        assert!(buffer.was_dash_executed(Motions::DashForward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_4_held_5_4_should_fail() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        assert!(
            !(buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false)
                && ctx.can_dash_b)
        );
    }

    #[test]
    fn dash_with_4_held_5_4_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=10 {
            dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        assert!(
            (buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false)
                && ctx.can_dash_b)
        );
    }

    #[test]
    fn dash_with_1_held_4_5_4_should_fail() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            // This prevents the empty neutral spaces to force a dash
            dash_helper(&mut buffer, Inputs::Down, &mut ctx, &false);
        }
        for _ in 20..=27 {
            dash_helper(&mut buffer, Inputs::DownBackward, &mut ctx, &false);
        }
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        assert!(
            !(buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false)
                && ctx.can_dash_b)
        );
    }

    #[test]
    fn dash_with_1_held_5_4_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        let mut ctx = SubContext::default();
        for _ in 1..=19 {
            // This prevents the empty neutral spaces to force a dash
            dash_helper(&mut buffer, Inputs::Down, &mut ctx, &false);
        }
        for _ in 20..=27 {
            dash_helper(&mut buffer, Inputs::DownBackward, &mut ctx, &false);
        }
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Neutral, &mut ctx, &false);
        dash_helper(&mut buffer, Inputs::Backward, &mut ctx, &false);
        assert!(
            (buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false)
                && ctx.can_dash_b)
        );
    }

    #[test]
    fn dash_with_1_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_2_in_the_middle_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Down);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_4_1_5_4_should_work() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }

    #[test]
    fn dash_with_4_5_1_4_should_fail() {
        let mut buffer = InputBuffer::default();
        test_helper(&mut buffer, Inputs::Backward);
        test_helper(&mut buffer, Inputs::Neutral);
        test_helper(&mut buffer, Inputs::DownBackward);
        test_helper(&mut buffer, Inputs::Backward);
        assert!(!buffer.was_dash_executed(Motions::DashBackward, buffer.dash, &false));
    }
}
