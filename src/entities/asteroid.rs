use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::f32::consts::PI;

use crate::components::chunk::Chunk;
use crate::components::types::Asteroid;
use crate::entities::entity::EntityBundle;
use crate::util::{from_polar, rotate, STURDINESS_CONSTANT};

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
    pub fn new(pos: RigidBodyPosition, vel: RigidBodyVelocity, scale: f32) -> AsteroidBundle {
        let asteroid_points = ASTEROID_SHAPE
            .clone()
            .into_iter()
            .map(|p| (p.0 * scale, p.1 * scale))
            .collect::<Vec<(f32, f32)>>();

        AsteroidBundle {
            base: EntityBundle::new(asteroid_points, pos, vel, 25.0),
            asteroid: Asteroid(scale),
        }
    }

    pub fn generate_daughters(
        pos: &RigidBodyPosition,
        vel: &RigidBodyVelocity,
        mass: &RigidBodyMassProps,
        scale: f32,
        sturdiness: f32,
    ) -> Vec<AsteroidBundle> {
        let NUM_DAUGHTERS: f32 = 3.;
        let BASE_WIDTH = 10.;
        let m = mass.local_mprops.inv_mass.recip();
        let v: Vec2 = vel.linvel.into();
        let posvec: Vec2 = pos.position.translation.into();
        let ke = 0.5 * m * v.length_squared();

        let pe = sturdiness.powf(2.0) / STURDINESS_CONSTANT; // Leftover potential energy after collision
        let total_energy = (ke + pe) / 10.0;

        let daughter_energy = total_energy / NUM_DAUGHTERS;
        let daughter_mass = m / NUM_DAUGHTERS;
        let daughter_speed = (2. * daughter_energy / daughter_mass).sqrt();
        let daughter_scale = (scale / NUM_DAUGHTERS).powi(2); // Surface area (and therefore density and mass) are scaled by scale^2

        let axis = v.normalize();
        let axis_angle = axis.angle_between(Vec2::new(1., 0.));

        vec![
            Self::new(
                RigidBodyPosition {
                    position: (
                        posvec
                            + rotate(
                                BASE_WIDTH * daughter_scale * from_polar(1.0, axis_angle),
                                PI / 2.,
                            ),
                        0.0,
                    )
                        .into(),
                    ..Default::default()
                },
                RigidBodyVelocity {
                    linvel: (rotate(axis, -0.2) * daughter_speed).into(),
                    angvel: 0.,
                },
                daughter_scale,
            ),
            Self::new(
                pos.clone(),
                RigidBodyVelocity {
                    linvel: (axis * daughter_speed).into(),
                    angvel: 0.,
                },
                daughter_scale,
            ),
            Self::new(
                RigidBodyPosition {
                    position: (
                        posvec
                            + rotate(
                                BASE_WIDTH * daughter_scale * from_polar(1.0, axis_angle),
                                -PI / 2.,
                            ),
                        0.0,
                    )
                        .into(),
                    ..Default::default()
                },
                RigidBodyVelocity {
                    linvel: (rotate(axis, 0.2) * daughter_speed).into(),
                    angvel: 0.,
                },
                daughter_scale,
            ),
        ]
    }

    pub fn spawn_for_chunk(commands: &mut Commands, chunk: &Chunk) {
        println!("Spawning for {:?}", chunk);
        let mut rng = rand::thread_rng();
        let num_distribution = Normal::new(3.0, 0.3).unwrap();
        let num_asteroids_in_chunk = (num_distribution.sample(&mut rng) as f64).round() as i32;
        let scale_distribution = Normal::new(4.0, 0.5).unwrap();

        for _ in 0..num_asteroids_in_chunk {
            let coords = chunk.random_point_inside(&mut rng);
            let vel = from_polar(rng.gen_range(0.0..100.0), rng.gen_range(0.0..(2.0 * PI)));
            commands.spawn_bundle(AsteroidBundle::new(
                coords.into(),
                RigidBodyVelocity {
                    linvel: vel.into(),
                    angvel: rng.gen_range(-1.0..1.0),
                },
                scale_distribution.sample(&mut rng),
            ));
        }
    }
}
