pub mod common;
pub mod custom;

pub use self::common::*;
pub use self::custom::*;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum States {
    Group(Group),
    Standing(Standing),
    Crouching(Crouching),
    Jumping(Jumping),
    Ken(ken::Ken),
}

impl States {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            States::Group(states) => states.set(buffer, ctx, physics),
            States::Standing(states) => states.set(buffer, ctx, physics),
            States::Crouching(states) => states.set(buffer, ctx, physics),
            States::Jumping(states) => states.set(buffer, ctx, physics),
            States::Ken(states) => states.set(buffer, ctx, physics),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Group {
    All,
    Normals,
    StNormals,
    CrNormals,
    AirNormals,
    Movement,
    Dashes,
    Walks,
    Jumps,
}

impl Group {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Group::All => {
                if Group::Normals.set(buffer, ctx, physics) {
                    return true;
                }
                if Group::Jumps.set(buffer, ctx, physics) {
                    return true;
                }
                if Group::Movement.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Group::Normals => {
                if Group::CrNormals.set(buffer, ctx, physics) {
                    return true;
                }
                if Group::StNormals.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }

            Group::StNormals => {
                if Standing::HeavyKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::HeavyPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::MediumKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::MediumPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::LightKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::LightPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }

            Group::CrNormals => {
                if Crouching::HeavyKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Crouching::HeavyPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Crouching::MediumKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Crouching::MediumPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Crouching::LightKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Crouching::LightPunch.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Group::AirNormals => {
                if Jumping::HeavyKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::HeavyPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::MediumKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::MediumPunch.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::LightKick.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::LightPunch.set(buffer, ctx, physics) {
                    return true;
                }

                false
            }
            Group::Movement => {
                if Standing::DashForward.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::DashBackward.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::WalkForward.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::WalkBackward.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Group::Dashes => {
                if Standing::DashForward.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::DashBackward.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Group::Walks => {
                if Standing::WalkForward.set(buffer, ctx, physics) {
                    return true;
                }
                if Standing::WalkBackward.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
            Group::Jumps => {
                if Jumping::Start.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::Forward.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::Backward.set(buffer, ctx, physics) {
                    return true;
                }
                if Jumping::Neutral.set(buffer, ctx, physics) {
                    return true;
                }
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Standing {
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
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Standing::LightPunch => {
                if buffer.buffered(Inputs::LightPunch, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::LightPunch));
                    return true;
                }
                false
            }
            Standing::MediumPunch => {
                if buffer.buffered(Inputs::MediumPunch, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::MediumPunch));
                    return true;
                }
                false
            }
            Standing::HeavyPunch => {
                if buffer.buffered(Inputs::HeavyPunch, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::HeavyPunch));
                    return true;
                }
                false
            }
            Standing::LightKick => {
                if buffer.buffered(Inputs::LightKick, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::LightKick));
                    return true;
                }
                false
            }
            Standing::MediumKick => {
                if buffer.buffered(Inputs::MediumKick, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::MediumKick));
                    return true;
                }
                false
            }
            Standing::HeavyKick => {
                if buffer.buffered(Inputs::HeavyKick, buffer.attack, &physics.facing_left)
                    && !down(buffer)
                {
                    ctx.next.replace(Box::new(standing::HeavyKick));
                    return true;
                }
                false
            }
            Standing::DashForward => {
                if buffer.was_dash_executed(Dashes::Forward, buffer.dash, &physics.facing_left)
                    && ctx.can_dash_f
                {
                    ctx.next.replace(Box::new(standing::DashForward));
                    return true;
                }
                false
            }
            Standing::DashBackward => {
                if buffer.was_dash_executed(Dashes::Backward, buffer.dash, &physics.facing_left)
                    && ctx.can_dash_b
                {
                    ctx.next.replace(Box::new(standing::DashBackward));
                    return true;
                }
                false
            }
            Standing::WalkForward => {
                if forward(buffer, &physics.facing_left) {
                    ctx.next.replace(Box::new(standing::WalkForward));
                    return true;
                }
                false
            }
            Standing::WalkBackward => {
                if backward(buffer, &physics.facing_left) {
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
    End,
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Crouching {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Crouching::Start => {
                if down(buffer) {
                    ctx.next.replace(Box::new(crouching::Start));
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
                if buffer.buffered(Inputs::LightPunch, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
                    ctx.next.replace(Box::new(crouching::LightPunch));
                    return true;
                }
                false
            }
            Crouching::MediumPunch => {
                if buffer.buffered(Inputs::MediumPunch, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
                    ctx.next.replace(Box::new(crouching::MediumPunch));
                    return true;
                }
                false
            }
            Crouching::HeavyPunch => {
                if buffer.buffered(Inputs::HeavyPunch, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
                    ctx.next.replace(Box::new(crouching::HeavyPunch));
                    return true;
                }
                false
            }
            Crouching::LightKick => {
                if buffer.buffered(Inputs::LightKick, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
                    ctx.next.replace(Box::new(crouching::LightKick));
                    return true;
                }
                false
            }
            Crouching::MediumKick => {
                if buffer.buffered(Inputs::MediumKick, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
                    ctx.next.replace(Box::new(crouching::MediumKick));
                    return true;
                }
                false
            }
            Crouching::HeavyKick => {
                if buffer.buffered(Inputs::HeavyKick, buffer.attack, &physics.facing_left)
                    && down(buffer)
                {
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
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
}

impl Jumping {
    pub fn set(&self, buffer: &InputBuffer, ctx: &mut SubContext, physics: &mut Physics) -> bool {
        match self {
            Jumping::Start => {
                if up(buffer) && !physics.airborne {
                    handle_jump_flags(ctx, buffer, physics);
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
                if up_forward(buffer, &physics.facing_left) {
                    ctx.next.replace(Box::new(jumping::Forward));
                    return true;
                }
            }
            Jumping::Backward => {
                if up_backward(buffer, &physics.facing_left) {
                    ctx.next.replace(Box::new(jumping::Backward));
                    return true;
                }
            }
            Jumping::End => {
                if !up(buffer) && physics.airborne {
                    ctx.next.replace(Box::new(jumping::End));
                    return true;
                }
            }
            Jumping::LightPunch => {
                if buffer.buffered(Inputs::LightPunch, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::LightPunch));
                    return true;
                }
            }
            Jumping::MediumPunch => {
                if buffer.buffered(Inputs::MediumPunch, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::MediumPunch));
                    return true;
                }
            }
            Jumping::HeavyPunch => {
                if buffer.buffered(Inputs::HeavyPunch, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::HeavyPunch));
                    return true;
                }
            }
            Jumping::LightKick => {
                if buffer.buffered(Inputs::LightKick, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::LightKick));
                    return true;
                }
            }
            Jumping::MediumKick => {
                if buffer.buffered(Inputs::MediumKick, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::MediumKick));
                    return true;
                }
            }
            Jumping::HeavyKick => {
                if buffer.buffered(Inputs::HeavyKick, buffer.attack, &physics.facing_left)
                    && physics.airborne
                {
                    ctx.next.replace(Box::new(jumping::HeavyKick));
                    return true;
                }
            }
        }
        false
    }
}
