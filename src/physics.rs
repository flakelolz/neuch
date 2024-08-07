use crate::prelude::*;
const DECELERATION: i32 = 1000;
const THRESHOLD: i32 = 1100;

#[derive(Debug, Clone, Copy, Default)]
pub struct Physics {
    /// Player's position on the world
    pub position: IVec2,
    /// Player's current velocity
    pub velocity: IVec2,
    /// Player's current acceleration
    pub acceleration: IVec2,
    /// Player's facing direction
    pub facing_left: bool,
    /// Whether or not the player is facing the opponent
    pub facing_opponent: bool,
    /// Whether or not the player is airborne
    pub airborne: bool,
    /// Characters pushbox width
    pub width: u32,
    /// Distance from the opponent
    pub distance: u32,
    /// Whether or not the player is cornered
    pub cornered: bool,
}

impl Physics {
    pub fn one() -> Self {
        Self {
            position: IVec2::from_screen(112, 0),
            velocity: IVec2::zero(),
            acceleration: IVec2::zero(),
            facing_left: false,
            facing_opponent: true,
            airborne: false,
            width: 0,
            distance: 0,
            cornered: false,
        }
    }

    pub fn two() -> Self {
        Self {
            position: IVec2::from_screen(304, 0),
            velocity: IVec2::zero(),
            acceleration: IVec2::zero(),
            facing_left: true,
            facing_opponent: true,
            airborne: false,
            width: 0,
            distance: 0,
            cornered: false,
        }
    }

    pub fn set_forward_velocity(&mut self, speed: i32) {
        self.velocity.x = if self.facing_left { -speed } else { speed };
    }

    pub fn set_forward_position(&mut self, pos: i32) {
        self.position.x += if self.facing_left { -pos } else { pos };
    }
}

pub fn physics_system(world: &mut World) {
    // Update facing direction
    let mut players = world
        .query_mut::<(&mut Physics, &Player)>()
        .into_iter()
        .collect::<Vec<_>>();

    let split = &mut players.split_at_mut(1);
    let (p1, p2) = split;
    if let Some((_, (player, _))) = p1.get_mut(0) {
        if let Some((_, (opponent, _))) = p2.get_mut(0) {
            // Make player 1 face the opponent
            player.facing_opponent = ((opponent.position.x <= player.position.x)
                && player.facing_left)
                || ((opponent.position.x >= player.position.x) && !player.facing_left);
            // Make player 2 face the opponent
            opponent.facing_opponent = ((player.position.x <= opponent.position.x)
                && opponent.facing_left)
                || ((player.position.x >= opponent.position.x) && !opponent.facing_left);

            // Calculate Player 1 distance
            player.distance = player.position.x.abs_diff(if player.facing_left {
                if player.facing_opponent {
                    opponent.position.x + (opponent.width / 2) as i32
                } else {
                    opponent.position.x - (opponent.width / 2) as i32
                }
            } else if player.facing_opponent {
                opponent.position.x - (opponent.width / 2) as i32
            } else {
                opponent.position.x + (opponent.width / 2) as i32
            });

            // Calculate Player 2 distance
            opponent.distance = opponent.position.x.abs_diff(if opponent.facing_left {
                if opponent.facing_opponent {
                    player.position.x + (player.width / 2) as i32
                } else {
                    player.position.x - (player.width / 2) as i32
                }
            } else if opponent.facing_opponent {
                player.position.x - (player.width / 2) as i32
            } else {
                player.position.x + (player.width / 2) as i32
            });
        }
    }
    // Update physics
    for (_, (physics, state)) in world.query_mut::<(&mut Physics, &mut StateMachine)>() {
        let reaction = &mut state.context.ctx.reaction;
        if reaction.hitstop == 0 {
            // Move position based on current velocity
            physics.position += physics.velocity;
            physics.velocity += physics.acceleration;

            // Apply knockback to the position
            if reaction.knockback != 0 {
                physics.position.x += reaction.knockback;

                // Decelerate
                if reaction.knockback > 0 {
                    reaction.knockback -= DECELERATION;
                }

                if reaction.knockback < 0 {
                    reaction.knockback += DECELERATION;
                }

                if reaction.knockback.abs() < THRESHOLD {
                    reaction.knockback = 0;
                }
            }
        }
    }
}

/// Conditionally flip the character to face the opponent if not already facing them.
pub fn face_opponent(physics: &mut Physics) -> bool {
    if !physics.facing_opponent {
        physics.facing_left = !physics.facing_left;
        physics.facing_opponent = true;
        return true;
    }
    false
}
