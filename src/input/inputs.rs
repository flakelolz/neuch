use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub backward: bool,
    pub forward: bool,
    pub lp: bool,
    pub mp: bool,
    pub hp: bool,
    pub lk: bool,
    pub mk: bool,
    pub hk: bool,
}

impl Input {
    pub fn update(&mut self, rl: &mut RaylibHandle, config: &InputConfig, player: &Player) {
        let port = match player {
            Player::One => 0,
            Player::Two => 1,
        };

        self.up = rl.is_key_down(config.keyboard.up)
            || rl.is_gamepad_button_down(port, config.gamepad.up);
        self.down = rl.is_key_down(config.keyboard.down)
            || rl.is_gamepad_button_down(port, config.gamepad.down);
        self.backward = rl.is_key_down(config.keyboard.left)
            || rl.is_gamepad_button_down(port, config.gamepad.left);
        self.forward = rl.is_key_down(config.keyboard.right)
            || rl.is_gamepad_button_down(port, config.gamepad.right);
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
