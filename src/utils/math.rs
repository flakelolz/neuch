#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::prelude::{GROUND_OFFSET, SCREEN_CENTER};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
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

impl IVec2 {
    pub fn from_screen(x: i32, y: i32) -> Self {
        Self {
            x: screen_to_world_num(x),
            y: screen_to_world_num(y),
        }
    }
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
