use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

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

pub fn new_asteroid_bundle() -> EntityBundle {
    let size = 4.0;
    let asteroid_points = ASTEROID_SHAPE
        .clone()
        .into_iter()
        .map(|p| (p.0 * size, p.1 * size))
        .collect::<Vec<(f32, f32)>>();
    EntityBundle::new(
        asteroid_points,
        Vec2::ZERO.into(),
        RigidBodyVelocity {
            linvel: Vec2::ZERO.into(),
            angvel: 1.0,
        },
    )
}
