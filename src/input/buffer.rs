use crate::prelude::*;

const BUFFER_SIZE: usize = 50;

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
    pub fn was_initially_pressed(&self, current: &Input, previous: &Input, flipped: &bool) -> bool {
        let forward = if *flipped {
            current.left && !previous.left
        } else {
            current.right && !previous.right
        };
        let backward = if *flipped {
            current.right && !previous.right
        } else {
            current.left && !previous.left
        };
        match self {
            Inputs::Up => current.up && !previous.up,
            Inputs::Down => current.down && !previous.down,
            Inputs::Forward => forward,
            Inputs::Backward => backward,
            Inputs::UpForward => current.up && !previous.up && forward,
            Inputs::UpBackward => current.up && !previous.up && backward,
            Inputs::DownForward => current.down && !previous.down && forward,
            Inputs::DownBackward => current.down && !previous.down && backward,
            Inputs::Neutral => {
                (!current.up && !current.down && !current.right && !current.left)
                    && (previous.up || previous.down || previous.right || previous.left)
            }
            Inputs::LightPunch => current.lp && !previous.lp,
            Inputs::MediumPunch => current.mp && !previous.mp,
            Inputs::HeavyPunch => current.hp && !previous.hp,
            Inputs::LightKick => current.lk && !previous.lk,
            Inputs::MediumKick => current.mk && !previous.mk,
            Inputs::HeavyKick => current.hk && !previous.hk,
        }
    }

    pub fn is_pressed(&self, current: &Input, flipped: &bool) -> bool {
        let forward = if *flipped {
            current.left
        } else {
            current.right
        };
        let backward = if *flipped {
            current.right
        } else {
            current.left
        };
        match self {
            Inputs::Up => current.up && !backward && !forward,
            Inputs::Down => current.down && !backward && !forward,
            Inputs::Forward => forward && !current.down && !current.up,
            Inputs::Backward => backward && !current.down && !current.up,
            Inputs::UpForward => current.up && forward,
            Inputs::UpBackward => current.up && backward,
            Inputs::DownForward => current.down && forward,
            Inputs::DownBackward => current.down && backward,
            Inputs::Neutral => !current.up && !current.down && !current.right && !current.left,
            Inputs::LightPunch => current.lp,
            Inputs::MediumPunch => current.mp,
            Inputs::HeavyPunch => current.hp,
            Inputs::LightKick => current.lk,
            Inputs::MediumKick => current.mk,
            Inputs::HeavyKick => current.hk,
        }
    }
    pub fn is_pressed_for_motion(&self, current: &Input, left: &bool, opponent: &bool) -> bool {
        let forward;
        let backward;

        if *left {
            if *opponent {
                forward = current.left;
                backward = current.right;
            } else {
                forward = current.right;
                backward = current.left;
            }
        } else if *opponent {
            forward = current.right;
            backward = current.left;
        } else {
            forward = current.left;
            backward = current.right;
        }
        match self {
            Inputs::Up => current.up && !backward && !forward,
            Inputs::Down => current.down && !backward && !forward,
            Inputs::Forward => forward && !current.down && !current.up,
            Inputs::Backward => backward && !current.down && !current.up,
            Inputs::UpForward => current.up && forward,
            Inputs::UpBackward => current.up && backward,
            Inputs::DownForward => current.down && forward,
            Inputs::DownBackward => current.down && backward,
            Inputs::Neutral => !current.up && !current.down && !current.right && !current.left,
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
    pub cancels: usize,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            index: BUFFER_SIZE - 1,
            buffer: [Input::default(); BUFFER_SIZE],
            dash: 8,
            attack: 2,
            cancels: 2,
        }
    }
}

impl InputBuffer {
    /// Moves the index forward and then adds the new input to the buffer
    pub fn update(&mut self, input: &Input) {
        self.index = (self.index + 1) % self.buffer.len();
        self.buffer[self.index] = *input;
    }

    /// Checks the current input
    pub fn current(&self) -> &Input {
        &self.buffer[self.index]
    }

    /// Checks the previous input
    pub fn previous(&self) -> &Input {
        &self.buffer[(self.buffer.len() + self.index - 1) % self.buffer.len()]
    }

    /// Check if the input is currently pressed
    pub fn pressed(&self, inputs: Inputs, flipped: &bool) -> bool {
        let current = self.current();
        inputs.is_pressed(current, flipped)
    }

    /// Check if the input was pressed on a specific frame
    fn pressed_on_frame(&self, inputs: Inputs, frame: usize, flipped: &bool) -> bool {
        let buffer_index = frame % self.buffer.len();
        let input_command = self.buffer[buffer_index];

        inputs.is_pressed(&input_command, flipped)
    }

    fn pressed_on_frame_exclusive(&self, inputs: Inputs, frame: usize, flipped: &bool) -> bool {
        let buffer_index = frame % self.buffer.len();
        let input_command = self.buffer[buffer_index];

        inputs.is_pressed_exclusive(&input_command, flipped)
    }

    /// Check if an input was performed within a certain duration on the past frames
    fn pressed_buffered(&self, input: Inputs, duration: usize, flipped: &bool) -> bool {
        for i in 0..duration + 1 {
            if self.just_pressed_on_frame(input, self.buffer.len() + self.index - i, flipped) {
                return true;
            }
        }

        false
    }

    /// Checks if the input was initially pressed this frame
    fn just_pressed(&self, inputs: Inputs, flipped: &bool) -> bool {
        let current = self.current();
        let previous = self.previous();

        inputs.was_initially_pressed(current, previous, flipped)
    }

    /// Checks if the input was initially pressed on a specific frame
    fn just_pressed_on_frame(&self, inputs: Inputs, frame: usize, flipped: &bool) -> bool {
        let buffer_index = frame % self.buffer.len();
        let last_index = (self.buffer.len() + frame - 1) % self.buffer.len();

        let current = self.buffer[buffer_index];
        let previous = self.buffer[last_index];

        inputs.was_initially_pressed(&current, &previous, flipped)
    }

    /// Check if an input was performed within a certain duration on the past frames
    pub fn buffered(&self, input: Inputs, duration: usize, flipped: &bool) -> bool {
        self.pressed_buffered(input, duration, flipped)
    }

    /// Check if an input has been held for a certain amount of frames
    pub fn held(&self, input: Inputs, duration: usize, flipped: &bool) -> bool {
        for i in 0..duration + 1 {
            if self.pressed_on_frame(input, self.buffer.len() + self.index - i, flipped) {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    pub fn held_exclusive(&self, input: Inputs, duration: usize, flipped: &bool) -> bool {
        for i in 0..duration + 1 {
            if self.pressed_on_frame_exclusive(input, self.buffer.len() + self.index - i, flipped) {
                continue;
            } else {
                return false;
            }
        }

        true
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

        let current = buffer.current();
        let previous = buffer.previous();

        let inputs = Inputs::Down;

        assert!(inputs.was_initially_pressed(current, previous, &false));
    }

    #[test]
    fn was_input_pressed() {
        let mut buffer = InputBuffer::default();

        for _ in 0..1 {
            let input = Input {
                right: true,
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
                right: true,
                ..Default::default()
            };

            buffer.update(&input);
        }

        assert!(buffer.just_pressed(Inputs::Forward, &false));
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

        assert!(buffer.just_pressed_on_frame(Inputs::Down, 1, &false));
        assert!(!buffer.just_pressed_on_frame(Inputs::Up, 1, &false));
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

        assert!(buffer.pressed_buffered(Inputs::Up, 2, &false));
        assert!(buffer.pressed_buffered(Inputs::Down, 5, &false));
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

        assert!(buffer.held(Inputs::Down, 4, &false));
    }
}
