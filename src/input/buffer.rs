use crate::prelude::*;

const BUFFER_SIZE: usize = 50;

#[derive(Debug, Clone, Copy)]
pub enum Motions {
    Qcf,
    Qcb,
    Dp,
    RDp,
    Hcf,
    Hcb,
    DashForward,
    DashBackward,
    ForcedDashForward,
    ForcedDashBackward,
}

impl Motions {
    pub fn notation(&self) -> Vec<Vec<u8>> {
        match self {
            Motions::DashForward => {
                vec![vec![6, 5, 6]]
            }
            Motions::DashBackward => {
                vec![vec![4, 5, 4]]
            }
            Motions::ForcedDashForward => {
                vec![vec![5, 6, 5, 6]]
            }
            Motions::ForcedDashBackward => {
                vec![vec![5, 4, 5, 4]]
            }
            Motions::Qcf => {
                vec![vec![2, 3, 6]]
            }
            Motions::Qcb => {
                vec![vec![2, 1, 4]]
            }
            Motions::Dp => {
                vec![vec![6, 2, 3], vec![3, 2, 3], vec![6, 3, 6]]
            }
            Motions::RDp => {
                vec![vec![4, 2, 1], vec![1, 2, 1], vec![4, 1, 4]]
            }
            Motions::Hcf => {
                vec![vec![4, 1, 2, 3, 6], vec![4, 1, 3, 6], vec![4, 2, 6]]
            }
            Motions::Hcb => {
                vec![vec![6, 3, 2, 1, 4], vec![6, 3, 1, 4], vec![6, 2, 4]]
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Inputs {
    Up,
    Down,
    Forward,
    Backward,
    UpForward,
    UpBackward,
    DownForward,
    DownBackward,
    Neutral,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Inputs {
    pub fn was_initially_pressed(&self, current: &Input, previous: &Input) -> bool {
        match self {
            Inputs::Up => current.up && !previous.up,
            Inputs::Down => current.down && !previous.down,
            Inputs::Forward => current.forward && !previous.forward,
            Inputs::Backward => current.backward && !previous.backward,
            Inputs::UpForward => current.up && current.forward && !previous.up && !previous.forward,
            Inputs::UpBackward => {
                current.up && current.backward && !previous.up && !previous.backward
            }
            Inputs::DownForward => {
                current.down && current.forward && !previous.down && !previous.forward
            }
            Inputs::DownBackward => {
                current.down && current.backward && !previous.down && !previous.backward
            }
            Inputs::Neutral => {
                (!current.up && !current.down && !current.forward && !current.backward)
                    && (previous.up || previous.down || previous.forward || previous.backward)
            }
            Inputs::LightPunch => current.lp && !previous.lp,
            Inputs::MediumPunch => current.mp && !previous.mp,
            Inputs::HeavyPunch => current.hp && !previous.hp,
            Inputs::LightKick => current.lk && !previous.lk,
            Inputs::MediumKick => current.mk && !previous.mk,
            Inputs::HeavyKick => current.hk && !previous.hk,
        }
    }

    pub fn is_pressed(&self, current: &Input) -> bool {
        match self {
            Inputs::Up => current.up,
            Inputs::Down => current.down,
            Inputs::Forward => current.forward,
            Inputs::Backward => current.backward,
            Inputs::UpForward => current.up && current.forward,
            Inputs::UpBackward => current.up && current.backward,
            Inputs::DownForward => current.down && current.forward,
            Inputs::DownBackward => current.down && current.backward,
            Inputs::Neutral => {
                !current.up && !current.down && !current.forward && !current.backward
            }
            Inputs::LightPunch => current.lp,
            Inputs::MediumPunch => current.mp,
            Inputs::HeavyPunch => current.hp,
            Inputs::LightKick => current.lk,
            Inputs::MediumKick => current.mk,
            Inputs::HeavyKick => current.hk,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InputBuffer {
    pub index: usize,
    pub buffer: [Input; BUFFER_SIZE],
    pub dash: usize,
    pub attack: usize,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            index: BUFFER_SIZE - 1,
            buffer: [Input::default(); BUFFER_SIZE],
            dash: 12,
            attack: 2,
        }
    }
}

impl InputBuffer {
    /// Moves the index forward and then adds the new input to the buffer
    pub fn update(&mut self, input: &Input) {
        self.index = (self.index + 1) % self.buffer.len();
        self.buffer[self.index] = *input;
    }

    fn get_curret_input(&self) -> Input {
        self.buffer[self.index]
    }

    fn get_previous_input(&self) -> Input {
        self.buffer[(self.buffer.len() + self.index - 1) % self.buffer.len()]
    }

    /// Check if the input is currently pressed
    fn is_input_pressed(&self, inputs: &Inputs) -> bool {
        let current = self.get_curret_input();

        inputs.is_pressed(&current)
    }

    /// Check if the input was pressed on a specific frame
    fn is_input_pressed_on_frame(&self, inputs: &Inputs, frame: usize) -> bool {
        let buffer_index = frame % self.buffer.len();
        let input_command = self.buffer[buffer_index];

        inputs.is_pressed(&input_command)
    }

    /// Check if an input has been held for a certain amount of frames
    fn is_input_held(&self, input: &Inputs, duration: usize) -> bool {
        for i in 0..duration + 1 {
            if self.is_input_pressed_on_frame(input, self.buffer.len() + self.index - i) {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    /// Checks if the input was initially pressed this frame
    fn was_input_pressed(&self, inputs: &Inputs) -> bool {
        let current = self.get_curret_input();
        let previous = self.get_previous_input();

        inputs.was_initially_pressed(&current, &previous)
    }

    /// Checks if the input was initially pressed on a specific frame
    fn was_input_pressed_on_frame(&self, inputs: &Inputs, frame: usize) -> bool {
        let buffer_index = frame % self.buffer.len();
        let last_index = (self.buffer.len() + frame - 1) % self.buffer.len();

        let current = self.buffer[buffer_index];
        let previous = self.buffer[last_index];

        inputs.was_initially_pressed(&current, &previous)
    }

    /// Check if an input was performed within a certain duration on the past frames
    fn was_input_pressed_buffered(&self, input: &Inputs, duration: usize) -> bool {
        for i in 0..duration + 1 {
            if self.was_input_pressed_on_frame(input, self.buffer.len() + self.index - i) {
                return true;
            }
        }

        false
    }

    /// Checks the current input
    pub fn input(&self) -> &Input {
        &self.buffer[self.index]
    }

    /// Checks the previous input
    pub fn previous(&self) -> &Input {
        &self.buffer[(self.buffer.len() + self.index - 1) % self.buffer.len()]
    }

    /// Check if an input was performed within a certain duration on the past frames
    pub fn buffered(&self, input: &Inputs, duration: usize) -> bool {
        self.was_input_pressed_buffered(input, duration)
    }

    /// Check if an input has been held for a certain amount of frames
    pub fn held(&self, input: &Inputs, duration: usize) -> bool {
        self.is_input_held(input, duration)
    }

    /// Check if a motion was performed within a time limit
    pub fn was_motion_executed(&self, motions: &Motions, mut time_limit: usize) -> bool {
        if time_limit > (self.buffer.len() + self.index) {
            time_limit = self.buffer.len() + self.index;
        }

        let motion_list = motions.notation();
        let mut current_motion_index = 0;

        for motion in motion_list {
            for count in 0..time_limit {
                let buffer_position =
                    (self.buffer.len() + self.index - (time_limit - 1) + count) % self.buffer.len();

                let input_command = self.buffer[buffer_position];
                let direction = motion[current_motion_index];

                if check_numpad_direction(&input_command, direction)
                    && !check_invalid_motion(motions, self, count)
                {
                    current_motion_index += 1;

                    if current_motion_index >= motion.len() {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn validate_dash(&mut self, ctx: &mut SubContext) {
        let duration = 8;
        if self.held(&Inputs::Forward, duration) && !self.held(&Inputs::DownForward, duration)
            || check_invalid_motion(&Motions::DashForward, self, self.dash)
        {
            ctx.can_dash_f = false;
        } else if self.held(&Inputs::Neutral, duration)
            || self.held(&Inputs::Backward, duration)
            || self.held(&Inputs::Down, duration)
            || self.held(&Inputs::Up, duration)
        {
            ctx.can_dash_f = true;
        }

        if self.held(&Inputs::Backward, duration) && !self.held(&Inputs::DownBackward, duration)
            || check_invalid_motion(&Motions::DashBackward, self, self.dash)
        {
            ctx.can_dash_b = false;
        } else if self.held(&Inputs::Neutral, duration)
            || self.held(&Inputs::Forward, duration)
            || self.held(&Inputs::Down, duration)
            || self.held(&Inputs::Up, duration)
        {
            ctx.can_dash_b = true;
        }
    }
}

/// Checks if a direction was pressed using numpad notation
fn check_numpad_direction(input: &Input, direction: u8) -> bool {
    match direction {
        1 => input.down && input.backward,
        2 => input.down && !(input.backward || input.forward),
        3 => input.down && input.forward,
        4 => input.backward && !(input.up || input.down),
        5 => !input.backward && !input.up && !input.down && !input.forward,
        6 => input.forward && !(input.up || input.down),
        7 => input.up && input.backward,
        8 => input.up && !(input.backward || input.forward),
        9 => input.up && input.forward,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
