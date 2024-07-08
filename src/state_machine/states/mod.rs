pub mod common;

pub use self::common::*;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum States {
    Group(Group),
    Standing(Standing),
    Crouching(Crouching),
    Jumping(Jumping),
}

impl Default for States {
    fn default() -> Self {
        Self::Standing(Standing::Idle)
    }
}

impl States {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext) -> bool {
        match self {
            States::Group(states) => states.set(buffer, ctx),
            States::Standing(states) => states.set(buffer, ctx),
            States::Crouching(states) => states.set(buffer, ctx),
            States::Jumping(states) => states.set(buffer, ctx),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Group {
    All,
    Normals,
    StNormals,
    CrNormals,
    Movement,
    Dashes,
    Walks,
    Jumps,
}

impl Group {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext) -> bool {
        match self {
            Group::All => {
                if Group::Normals.set(buffer, ctx) {
                    return true;
                }
                if Group::Movement.set(buffer, ctx) {
                    return true;
                }
                if Group::Jumps.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::Normals => {
                if Group::CrNormals.set(buffer, ctx) {
                    return true;
                }
                if Group::StNormals.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::StNormals => {
                if Standing::HeavyKick.set(buffer, ctx) {
                    return true;
                }
                if Standing::HeavyPunch.set(buffer, ctx) {
                    return true;
                }
                if Standing::MediumKick.set(buffer, ctx) {
                    return true;
                }
                if Standing::MediumPunch.set(buffer, ctx) {
                    return true;
                }
                if Standing::LightKick.set(buffer, ctx) {
                    return true;
                }
                if Standing::LightPunch.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::CrNormals => {
                if Crouching::HeavyKick.set(buffer, ctx) {
                    return true;
                }
                if Crouching::HeavyPunch.set(buffer, ctx) {
                    return true;
                }
                if Crouching::MediumKick.set(buffer, ctx) {
                    return true;
                }
                if Crouching::MediumPunch.set(buffer, ctx) {
                    return true;
                }
                if Crouching::LightKick.set(buffer, ctx) {
                    return true;
                }
                if Crouching::LightPunch.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::Movement => {
                if Standing::DashForward.set(buffer, ctx) {
                    return true;
                }
                if Standing::DashBackward.set(buffer, ctx) {
                    return true;
                }
                if Standing::WalkForward.set(buffer, ctx) {
                    return true;
                }
                if Standing::WalkBackward.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::Dashes => {
                if Standing::DashForward.set(buffer, ctx) {
                    return true;
                }
                if Standing::DashBackward.set(buffer, ctx) {
                    return true;
                }
                false
            }

            Group::Walks => {
                if Standing::WalkForward.set(buffer, ctx) {
                    return true;
                }
                if Standing::WalkBackward.set(buffer, ctx) {
                    return true;
                }
                false
            }
            Group::Jumps => {
                if Jumping::Start.set(buffer, ctx) {
                    return true;
                }
                if Jumping::Forward.set(buffer, ctx) {
                    return true;
                }
                if Jumping::Backward.set(buffer, ctx) {
                    return true;
                }
                if Jumping::Neutral.set(buffer, ctx) {
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
    WalkForward,
    WalkBackward,
}

impl Standing {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext) -> bool {
        match self {
            Standing::Idle => {
                if neutral(buffer) {
                    ctx.next.replace(Box::new(standing::Idle));
                    return true;
                }
                false
            }
            Standing::LightPunch => {
                if buffer.buffered(&Inputs::LightPunch, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::LightPunch));
                    return true;
                }
                false
            }
            Standing::MediumPunch => {
                if buffer.buffered(&Inputs::MediumPunch, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::MediumPunch));
                    return true;
                }
                false
            }
            Standing::HeavyPunch => {
                if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::HeavyPunch));
                    return true;
                }
                false
            }
            Standing::LightKick => {
                if buffer.buffered(&Inputs::LightKick, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::LightKick));
                    return true;
                }
                false
            }
            Standing::MediumKick => {
                if buffer.buffered(&Inputs::MediumKick, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::MediumKick));
                    return true;
                }
                false
            }
            Standing::HeavyKick => {
                if buffer.buffered(&Inputs::HeavyKick, buffer.attack) && !down(buffer) {
                    ctx.next.replace(Box::new(standing::HeavyKick));
                    return true;
                }
                false
            }
            Standing::DashForward => {
                if buffer.was_motion_executed(&Motions::ForcedDashForward, buffer.dash)
                    && !check_invalid_motion(&Motions::DashForward, buffer, buffer.dash)
                {
                    ctx.next.replace(Box::new(standing::DashForward));
                    return true;
                }
                if buffer.was_motion_executed(&Motions::DashForward, buffer.dash)
                    && ctx.can_dash_f
                {
                    ctx.next.replace(Box::new(standing::DashForward));
                    return true;
                }
                false
            }
            Standing::DashBackward => {
                if buffer.was_motion_executed(&Motions::ForcedDashBackward, buffer.dash)
                    && !check_invalid_motion(&Motions::DashBackward, buffer, buffer.dash)
                {
                    ctx.next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                if buffer.was_motion_executed(&Motions::DashBackward, buffer.dash)
                    && ctx.can_dash_b
                {
                    ctx.next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                false
            }
            Standing::WalkForward => {
                if forward(buffer) {
                    ctx.next.replace(Box::new(standing::WalkForward));
                    return true;
                }
                false
            }
            Standing::WalkBackward => {
                if backward(buffer) {
                    ctx.next.replace(Box::new(standing::WalkBackward));
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
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext) -> bool {
        match self {
            Crouching::Start => {
                if down(buffer) {
                    ctx.next.replace(Box::new(crouching::Start));
                    return true;
                }
                false
            }
            Crouching::Idle => {
                if down(buffer) {
                    ctx.next.replace(Box::new(crouching::Idle));
                    return true;
                }
                false
            }
            Crouching::End => {
                if !down(buffer) {
                    ctx.next.replace(Box::new(crouching::End));
                    return true;
                }
                false
            }
            Crouching::LightPunch => {
                if buffer.buffered(&Inputs::LightPunch, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::LightPunch));
                    return true;
                }
                false
            }
            Crouching::MediumPunch => {
                if buffer.buffered(&Inputs::MediumPunch, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::MediumPunch));
                    return true;
                }
                false
            }
            Crouching::HeavyPunch => {
                if buffer.buffered(&Inputs::HeavyPunch, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::HeavyPunch));
                    return true;
                }
                false
            }
            Crouching::LightKick => {
                if buffer.buffered(&Inputs::LightKick, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::LightKick));
                    return true;
                }
                false
            }
            Crouching::MediumKick => {
                if buffer.buffered(&Inputs::MediumKick, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::MediumKick));
                    return true;
                }
                false
            }
            Crouching::HeavyKick => {
                if buffer.buffered(&Inputs::HeavyKick, buffer.attack) && down(buffer) {
                    ctx.next.replace(Box::new(crouching::HeavyKick));
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Jumping {
    Start,
    Neutral,
    Forward,
    Backward,
    End,
}

impl Jumping {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext) -> bool {
        match self {
            Jumping::Start => {
                if up(buffer) && !ctx.airborne {
                    ctx.next.replace(Box::new(jumping::Start));
                    return true;
                }
            }
            Jumping::Neutral => {
                if up(buffer) {
                    ctx.next.replace(Box::new(jumping::Neutral));
                    return true;
                }
            }
            Jumping::Forward => {
                if up_forward(buffer) {
                    ctx.next.replace(Box::new(jumping::Forward));
                    return true;
                }
            }
            Jumping::Backward => {
                if up_backward(buffer) {
                    ctx.next.replace(Box::new(jumping::Backward));
                    return true;
                }
            }
            Jumping::End => {
                if !up(buffer) && ctx.airborne {
                    ctx.next.replace(Box::new(jumping::End));
                    return true;
                }
            }
        }
        false
    }
}
