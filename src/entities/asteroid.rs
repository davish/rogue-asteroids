use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;
use rand::Rng;
use std::f32::consts::PI;

use crate::components::chunk::Chunk;
use crate::components::types::Asteroid;
use crate::entities::entity::EntityBundle;
use crate::util::from_polar;

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
            base: EntityBundle::new(asteroid_points, pos.into(), vel.into(), 25.0),
            asteroid: Asteroid {},
        }
    }

    pub fn spawn_for_chunk(commands: &mut Commands, chunk: &Chunk) {
        println!("Spawning for {:?}", chunk);
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            let coords = chunk.random_point_inside(&mut rng);
            let vel = from_polar(rng.gen_range(0.0..100.0), rng.gen_range(0.0..(2.0 * PI)));
            commands.spawn_bundle(AsteroidBundle::new(
                coords.into(),
                RigidBodyVelocity {
                    linvel: vel.into(),
                    angvel: rng.gen_range(-1.0..1.0),
                },
            ));
        }
    }
}
