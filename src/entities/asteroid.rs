use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::components::types::Asteroid;
use crate::entities::entity::EntityBundle;

lazy_static! {
    static ref ASTEROID_SHAPE: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (-1.0, -3.0),
        (-5.0, -2.0),
        (-5.0, 1.0),
        (-1.0, 4.0),
        (3.0, 4.0),
        (3.0, -1.0)
    ];
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    #[bundle]
    base: EntityBundle,

    asteroid: Asteroid,
}

impl AsteroidBundle {
    pub fn new(pos: RigidBodyPosition, vel: RigidBodyVelocity) -> AsteroidBundle {
        let size = 4.0;
        let asteroid_points = ASTEROID_SHAPE
            .clone()
            .into_iter()
            .map(|p| (p.0 * size, p.1 * size))
            .collect::<Vec<(f32, f32)>>();

        AsteroidBundle {
            base: EntityBundle::new(asteroid_points, pos, vel),
            asteroid: Asteroid {},
        }
    }
}
