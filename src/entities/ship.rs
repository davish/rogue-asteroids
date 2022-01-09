use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use super::entity::EntityBundle;

use crate::components::ship::*;

lazy_static! {
    static ref SPACESHIP_SHAPE: Vec<(f32, f32)> = vec![(-6.0, -10.0), (0.0, 14.0), (6.0, -10.0),];
    pub static ref EXHAUST_SHAPE: Vec<(f32, f32)> = vec![(-3.0, -12.0), (0.0, -20.0), (3.0, -12.0)];
}

#[derive(Bundle)]
pub struct ThrusterBundle {
    #[bundle]
    base: ShapeBundle,

    thruster: Thruster,
}

#[derive(Bundle)]
pub struct Ship {
    #[bundle]
    base: EntityBundle,

    engines: Engines,
    controls: Controls,
    fuel: Fuel,
}

impl Ship {
    pub fn new(pos: RigidBodyPosition) -> Self {
        Self {
            base: EntityBundle::new(
                SPACESHIP_SHAPE.clone(),
                pos,
                RigidBodyVelocity {
                    linvel: Vec2::ZERO.into(),
                    angvel: 0.0,
                },
                100.0,
            ),
            engines: Engines {
                thrust: 50000.0,
                spin: 5.0,
            },
            controls: Default::default(),
            fuel: Fuel(100.0),
        }
    }
}
