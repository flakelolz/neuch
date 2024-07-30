use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motions {
    Qcf,
    Qcb,
    Dpf,
    Dpb,
    Hcf,
    Hcb,
}

impl Motions {
    pub fn notation(&self) -> Vec<Vec<u8>> {
        match self {
            Motions::Qcf => {
                vec![vec![2, 3, 6]]
            }
            Motions::Qcb => {
                vec![vec![2, 1, 4]]
            }
            Motions::Dpf => {
                vec![vec![6, 2, 3], vec![2, 3, 2, 3]]
            }
            Motions::Dpb => {
                vec![vec![4, 2, 1], vec![2, 1, 2, 1]]
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

impl InputBuffer {
    // Checks if a motion was executed with less than 10 frames between each step of the motion and
    // the button
    pub fn was_motion_executed(&self, motions: Motions, button: Inputs) -> bool {
        let motion_list = motions.notation();
        let mut motion_index;
        let limit = 9;

        let mut translated = Vec::with_capacity(9);
        for motion in motion_list.iter() {
            // Pointer to the end of the slice
            let mut right = self.index;
            // Left if looking 9 frames into the past of the buffer
            let mut left = (self.buffer.len() + right - (limit - 1) - 1) % self.buffer.len();
            // Make the attack button part of the motion and the 9 frame limit
            translated.push(button);

            // Translate the &[u8] to actual inputs
            for direction in motion.iter().rev() {
                translated.push(numpad_to_inputs(*direction));
            }

            motion_index = 0;

            for _ in translated.iter() {
                // Buffer slice of the last 9 inputs
                let slice = if left > right {
                    // When left is greater than right take whats everything from left pointer to
                    // the end and everything from 0 to right pointer and contactenate them
                    let left_slice = &self.buffer[left..];
                    let right_slice = &self.buffer[..=right];
                    [left_slice, right_slice].concat()
                } else {
                    self.buffer[left..=right].to_vec()
                };

                // Inputs in the motion
                let inputs = &translated[motion_index];

                for (i, current) in slice.iter().rev().enumerate() {
                    if inputs.is_pressed(current, &current.facing_left) {
                        // Update buffer slice based on where the input was found
                        right = (self.buffer.len() + right - i) % self.buffer.len();
                        left = (self.buffer.len() + right - (limit - 1) - 1) % self.buffer.len();
                        // Update input for the motion
                        motion_index += 1;
                        break;
                    }
                }
                if motion_index >= translated.len() {
                    return true;
                }
            }
            // Clear the translation layer for the next motion
            translated.clear();
        }

        false
    }

    /// Checks only on the directions given as a &[u8] instead of the notation list of each motion
    pub fn was_motion_executed_exact(&self, motion: &[u8], button: Inputs) -> bool {
        let mut motion_index;
        let limit = 9;

        let mut translated = Vec::with_capacity(9);
            // Pointer to the end of the slice
            let mut right = self.index;
            // Left if looking 9 frames into the past of the buffer
            let mut left = (self.buffer.len() + right - (limit - 1) - 1) % self.buffer.len();
            // Make the attack button part of the motion and the 9 frame limit
            translated.push(button);

            // Translate the &[u8] to actual inputs
            for direction in motion.iter().rev() {
                translated.push(numpad_to_inputs(*direction));
            }

            motion_index = 0;

            for _ in translated.iter() {
                // Buffer slice of the last 9 inputs
                let slice = if left > right {
                    // When left is greater than right take whats everything from left pointer to
                    // the end and everything from 0 to right pointer and contactenate them
                    let left_slice = &self.buffer[left..];
                    let right_slice = &self.buffer[..=right];
                    [left_slice, right_slice].concat()
                } else {
                    self.buffer[left..=right].to_vec()
                };

                // Inputs in the motion
                let inputs = &translated[motion_index];

                for (i, current) in slice.iter().rev().enumerate() {
                    if inputs.is_pressed(current, &current.facing_left) {
                        // Update buffer slice based on where the input was found
                        right = (self.buffer.len() + right - i) % self.buffer.len();
                        left = (self.buffer.len() + right - (limit - 1) - 1) % self.buffer.len();
                        // Update input for the motion
                        motion_index += 1;
                        break;
                    }
                }
                if motion_index >= translated.len() {
                    return true;
                }
            }

        false
    }

    /// Check if a motion was performed within a time limit
    pub fn was_motion_executed_in_time(&self, motions: Motions, mut time_limit: usize) -> bool {
        if time_limit > (self.buffer.len() + self.index) {
            time_limit = self.buffer.len() + self.index;
        }

        let motion_list = motions.notation();
        let mut current_motion_index;

        for motion in &motion_list {
            current_motion_index = 0;

            for count in 0..time_limit {
                let buffer_position =
                    (self.buffer.len() + self.index - (time_limit - 1) + count) % self.buffer.len();

                let input_command = self.buffer[buffer_position];
                let direction = motion[current_motion_index];

                if check_numpad_direction(&input_command, direction, &input_command.facing_left) {
                    current_motion_index += 1;
                    if current_motion_index >= motion.len() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

/// Checks if a direction was pressed using numpad notation
pub fn check_numpad_direction(input: &Input, direction: u8, flipped: &bool) -> bool {
    let forward = if *flipped { input.left } else { input.right };
    let backward = if *flipped { input.right } else { input.left };
    match direction {
        1 => input.down && backward,
        2 => input.down && !backward && !forward,
        3 => input.down && forward,
        4 => backward && !input.up && !input.down,
        5 => !backward && !input.up && !input.down && !forward,
        6 => forward && !input.up && !input.down,
        7 => input.up && backward,
        8 => input.up && !backward && !forward,
        9 => input.up && forward,
        _ => false,
    }
}

pub fn numpad_to_inputs(numpad: u8) -> Inputs {
    match numpad {
        1 => Inputs::DownBackward,
        2 => Inputs::Down,
        3 => Inputs::DownForward,
        4 => Inputs::Backward,
        5 => Inputs::Neutral,
        6 => Inputs::Forward,
        7 => Inputs::UpBackward,
        8 => Inputs::Up,
        9 => Inputs::UpForward,
        _ => Inputs::Neutral,
    }
}
