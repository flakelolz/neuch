use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub forward: bool,
    pub backward: bool,
    pub lp: bool,
    pub mp: bool,
    pub hp: bool,
    pub lk: bool,
    pub mk: bool,
    pub hk: bool,
}

impl Input {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        config: &InputConfig,
        player: &Player,
        flipped: bool,
    ) {
        let port = match player {
            Player::One => 0,
            Player::Two => 1,
        };

        self.up = rl.is_key_down(config.keyboard.up)
            || rl.is_gamepad_button_down(port, config.gamepad.up);
        self.down = rl.is_key_down(config.keyboard.down)
            || rl.is_gamepad_button_down(port, config.gamepad.down);
        // Forward
        {
            if rl.is_key_down(config.keyboard.right)
                || rl.is_gamepad_button_down(port, config.gamepad.right)
            {
                if flipped {
                    self.backward = true;
                } else {
                    self.forward = true;
                }
            }
        }
        // Backward
        {
            if rl.is_key_down(config.keyboard.left)
                || rl.is_gamepad_button_down(port, config.gamepad.left)
            {
                if flipped {
                    self.forward = true;
                } else {
                    self.backward = true;
                }
            }
        }
        self.lp = rl.is_key_down(config.keyboard.lp)
            || rl.is_gamepad_button_down(port, config.gamepad.lp);
        self.mp = rl.is_key_down(config.keyboard.mp)
            || rl.is_gamepad_button_down(port, config.gamepad.mp);
        self.hp = rl.is_key_down(config.keyboard.hp)
            || rl.is_gamepad_button_down(port, config.gamepad.hp);
        self.lk = rl.is_key_down(config.keyboard.lk)
            || rl.is_gamepad_button_down(port, config.gamepad.lk);
        self.mk = rl.is_key_down(config.keyboard.mk)
            || rl.is_gamepad_button_down(port, config.gamepad.mk);
        self.hk = rl.is_key_down(config.keyboard.hk)
            || rl.is_gamepad_button_down(port, config.gamepad.hk);

        // Horizontal Neutral SOCD
        if self.backward && self.forward {
            self.backward = false;
            self.forward = false;
        }

        // Up priority SOCD
        if self.up && self.down {
            self.up = true;
            self.down = false;
        }
    }
}
