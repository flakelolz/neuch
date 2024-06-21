use raylib::consts::{GamepadButton, KeyboardKey};

use crate::prelude::*;

pub fn update_inputs(world: &mut World, rl: &mut RaylibHandle) {
    world
        .query_mut::<(&mut Input, &InputConfig, &Player)>()
        .into_iter()
        .for_each(|(_, (input, config, player))| {
            input.update(rl, config, player);
        });
}

pub struct Keyboard {
    pub up: KeyboardKey,
    pub down: KeyboardKey,
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub lp: KeyboardKey,
    pub mp: KeyboardKey,
    pub hp: KeyboardKey,
    pub lk: KeyboardKey,
    pub mk: KeyboardKey,
    pub hk: KeyboardKey,
}

impl Keyboard {
    pub fn one() -> Self {
        Self {
            up: KeyboardKey::KEY_SPACE,
            down: KeyboardKey::KEY_S,
            left: KeyboardKey::KEY_A,
            right: KeyboardKey::KEY_D,
            lp: KeyboardKey::KEY_U,
            mp: KeyboardKey::KEY_I,
            hp: KeyboardKey::KEY_O,
            lk: KeyboardKey::KEY_J,
            mk: KeyboardKey::KEY_K,
            hk: KeyboardKey::KEY_L,
        }
    }

    pub fn two() -> Self {
        Self {
            up: KeyboardKey::KEY_UP,
            down: KeyboardKey::KEY_DOWN,
            left: KeyboardKey::KEY_LEFT,
            right: KeyboardKey::KEY_RIGHT,
            lp: KeyboardKey::KEY_ONE,
            mp: KeyboardKey::KEY_TWO,
            hp: KeyboardKey::KEY_THREE,
            lk: KeyboardKey::KEY_FOUR,
            mk: KeyboardKey::KEY_FIVE,
            hk: KeyboardKey::KEY_SIX,
        }
    }
}

pub struct Gamepad {
    pub up: GamepadButton,
    pub down: GamepadButton,
    pub left: GamepadButton,
    pub right: GamepadButton,
    pub lp: GamepadButton,
    pub mp: GamepadButton,
    pub hp: GamepadButton,
    pub lk: GamepadButton,
    pub mk: GamepadButton,
    pub hk: GamepadButton,
}

impl Gamepad {
    pub fn new() -> Self {
        Self {
            up: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP,
            down: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN,
            left: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT,
            right: GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
            lp: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
            mp: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP,
            hp: GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
            lk: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
            mk: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
            hk: GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
        }
    }
}

pub struct InputConfig {
    keyboard: Keyboard,
    gamepad: Gamepad,
}

impl InputConfig {
    pub fn one() -> Self {
        Self {
            keyboard: Keyboard::one(),
            gamepad: Gamepad::new(),
        }
    }

    pub fn two() -> Self {
        Self {
            keyboard: Keyboard::two(),
            gamepad: Gamepad::new(),
        }
    }
}

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
    }
}
