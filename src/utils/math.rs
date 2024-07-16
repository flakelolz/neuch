#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn from_screen(x: i32, y: i32) -> Self {
        Self {
            x: screen_to_world_num(x),
            y: screen_to_world_num(y),
        }
    }
}

impl std::ops::Add<IVec2> for IVec2 {
    type Output = IVec2;
    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<i32> for IVec2 {
    type Output = IVec2;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<i32> for IVec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Neg for IVec2 {
    type Output = IVec2;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::SubAssign<IVec2> for IVec2 {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn world_to_screen_num(coord: i32) -> i32 {
    coord / 1000
}

pub fn screen_to_world_num(coord: i32) -> i32 {
    coord * 1000
}

pub fn world_to_screen(coord: IVec2) -> (i32, i32) {
    (coord.x / 1000, coord.y / 1000)
}

pub fn screen_to_world(coord: IVec2) -> (i32, i32) {
    (coord.x * 1000, coord.y * 1000)
}

pub fn pos_to_screen(coord: IVec2) -> (i32, i32) {
    (
        world_to_screen_num(coord.x),
        -world_to_screen_num(coord.y) + GROUND_OFFSET,
    )
}

pub fn world_to_sprite_to_ui_num(coord: i32) -> i32 {
    let num = world_to_screen_num(coord);
    sprite_to_ui_num(num)
}

pub fn sprite_to_ui_num(x: i32) -> i32 {
    let x = x as f32;
    ((x / WIDTH_3S as f32) * WIDTH as f32) as i32
}

/// Translate from the sprite (416x234) layer to base resolution (1280x720).
pub fn sprite_to_ui(x: i32, y: i32) -> (i32, i32) {
    let old_x = x as f32;
    let old_y = y as f32;
    (
        ((old_x / WIDTH_3S as f32) * WIDTH as f32) as i32,
        ((old_y / HEIGHT_3S as f32) * HEIGHT as f32) as i32,
    )
}
