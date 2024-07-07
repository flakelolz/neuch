pub mod common;

pub use self::common::*;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum States {
    Group(Group),
    Standing(Standing),
    Crouching(Crouching),
}

impl Default for States {
    fn default() -> Self {
        Self::Standing(Standing::Idle)
    }
}

impl States {
    pub fn set(&self, buffer: &InputBuffer, next: &mut Option<Box<dyn State>>) -> bool {
        match self {
            Self::Group(states) => states.set(buffer, next),
            Self::Standing(states) => states.set(buffer, next),
            Self::Crouching(states) => states.set(buffer, next),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Group {
    All,
    Normals,
    Movement,
}

impl Group {
    pub fn set(&self, buffer: &InputBuffer, next: &mut Option<Box<dyn State>>) -> bool {
        match self {
            Group::All => {
                if Self::Normals.set(buffer, next) {
                    return true;
                }
                if Self::Movement.set(buffer, next) {
                    return true;
                }
                false
            }

            Group::Normals => {
                if Crouching::HeavyKick.set(buffer, next) {
                    return true;
                }
                if Crouching::HeavyPunch.set(buffer, next) {
                    return true;
                }
                if Crouching::MediumKick.set(buffer, next) {
                    return true;
                }
                if Crouching::MediumPunch.set(buffer, next) {
                    return true;
                }
                if Crouching::LightKick.set(buffer, next) {
                    return true;
                }
                if Crouching::LightPunch.set(buffer, next) {
                    return true;
                }
                if Standing::HeavyKick.set(buffer, next) {
                    return true;
                }
                if Standing::HeavyPunch.set(buffer, next) {
                    return true;
                }
                if Standing::MediumKick.set(buffer, next) {
                    return true;
                }
                if Standing::MediumPunch.set(buffer, next) {
                    return true;
                }
                if Standing::LightKick.set(buffer, next) {
                    return true;
                }
                if Standing::LightPunch.set(buffer, next) {
                    return true;
                }

                false
            }

            Group::Movement => {
                if Standing::DashForward.set(buffer, next) {
                    return true;
                }
                if Standing::DashBackward.set(buffer, next) {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Standing {
    Idle,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    DashForward,
    DashBackward,
}

impl Standing {
    pub fn set(&self, buffer: &InputBuffer, next: &mut Option<Box<dyn State>>) -> bool {
        match self {
            Standing::Idle => {
                if neutral(buffer) {
                    next.replace(Box::new(standing::Idle));
                    return true;
                }
                false
            }
            Standing::LightPunch => {
                if buffer.buffered(&Inputs::LightPunch, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::LightPunch));
                    return true;
                }
                false
            }
            Standing::MediumPunch => {
                if buffer.buffered(&Inputs::MediumPunch, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::MediumPunch));
                    return true;
                }
                false
            }
            Standing::HeavyPunch => {
                if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::HeavyPunch));
                    return true;
                }
                false
            }
            Standing::LightKick => {
                if buffer.buffered(&Inputs::LightKick, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::LightKick));
                    return true;
                }
                false
            }
            Standing::MediumKick => {
                if buffer.buffered(&Inputs::MediumKick, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::MediumKick));
                    return true;
                }
                false
            }
            Standing::HeavyKick => {
                if buffer.buffered(&Inputs::HeavyKick, buffer.attack) && neutral(buffer) {
                    next.replace(Box::new(standing::HeavyKick));
                    return true;
                }
                false
            }
            Standing::DashForward => {
                if buffer.was_motion_executed(Motions::ForcedDashForward, buffer.dash)
                    && !check_invalid_motion(Motions::DashForward, buffer, buffer.dash)
                {
                    next.replace(Box::new(standing::DashForward));
                    return true;
                }
                if buffer.was_motion_executed(Motions::DashForward, buffer.dash)
                    && buffer.can_dash_f
                {
                    next.replace(Box::new(standing::DashForward));
                    return true;
                }
                false
            }
            Standing::DashBackward => {
                if buffer.was_motion_executed(Motions::ForcedDashBackward, buffer.dash)
                    && !check_invalid_motion(Motions::DashBackward, buffer, buffer.dash)
                {
                    next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                if buffer.was_motion_executed(Motions::DashBackward, buffer.dash)
                    && buffer.can_dash_b
                {
                    next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Crouching {
    Start,
    Idle,
    End,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Crouching {
    pub fn set(&self, buffer: &InputBuffer, next: &mut Option<Box<dyn State>>) -> bool {
        match self {
            Crouching::Start => {
                if down(buffer) {
                    next.replace(Box::new(crouching::Start));
                    return true;
                }
                false
            }
            Crouching::Idle => {
                if down(buffer) {
                    next.replace(Box::new(crouching::Idle));
                    return true;
                }
                false
            }
            Crouching::End => {
                if neutral(buffer) {
                    next.replace(Box::new(crouching::End));
                    return true;
                }
                false
            }
            Crouching::LightPunch => {
                if buffer.buffered(&Inputs::LightPunch, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::LightPunch));
                    return true;
                }
                false
            }
            Crouching::MediumPunch => {
                if buffer.buffered(&Inputs::MediumPunch, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::MediumPunch));
                    return true;
                }
                false
            }
            Crouching::HeavyPunch => {
                if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::HeavyPunch));
                    return true;
                }
                false
            }
            Crouching::LightKick => {
                if buffer.buffered(&Inputs::LightKick, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::LightKick));
                    return true;
                }
                false
            }
            Crouching::MediumKick => {
                if buffer.buffered(&Inputs::MediumKick, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::MediumKick));
                    return true;
                }
                false
            }
            Crouching::HeavyKick => {
                if buffer.buffered(&Inputs::HeavyKick, buffer.attack) && down(buffer) {
                    next.replace(Box::new(crouching::HeavyKick));
                    return true;
                }
                false
            }
        }
    }
}
