use crate::prelude::*;

pub struct Physics {
    pub position: IVec2,
    pub velocity: IVec2,
    pub acceleration: IVec2,
}

impl Physics {
    pub fn one() -> Self {
        Self {
            position: IVec2 {
                x: WIDTH / 2,
                y: HEIGHT / 2,
            },
            velocity: IVec2 { x: 0, y: 0 },
            acceleration: IVec2 { x: 0, y: 0 },
        }
    }

    pub fn two() -> Self {
        Self {
            position: IVec2 { x: 0, y: 0 },
            velocity: IVec2 { x: 0, y: 0 },
            acceleration: IVec2 { x: 0, y: 0 },
        }
    }
}

pub fn update_physics(world: &mut World) {
    for (_, physics) in world.query_mut::<&mut Physics>() {
        physics.position += physics.velocity;
        physics.velocity += physics.acceleration;
    }
}
