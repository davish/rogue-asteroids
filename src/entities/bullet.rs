use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use super::entity::*;

lazy_static! {
    static ref BULLET_SHAPE: Vec<(f32, f32)> =
        vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];
}

pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    base: EntityBundle,
    launch_time: SpawnedAt,
    despawn_after: DespawnAfter,
    bullet: Bullet,
}

impl BulletBundle {
    pub fn launch_from(pos: &RigidBodyPosition, vel: &RigidBodyVelocity, launch_time: f64) -> Self {
        let angle = pos.position.rotation.angle();
        let launch_vel = Vec2::new(-angle.sin(), angle.cos()) * 200.0 + vel.linvel.into();
        Self {
            base: EntityBundle::new(
                BULLET_SHAPE.clone(),
                pos.clone(),
                RigidBodyVelocity {
                    linvel: launch_vel.into(),
                    angvel: 0.0,
                },
            ),
            launch_time: SpawnedAt(launch_time),
            despawn_after: DespawnAfter(3.0),
            bullet: Bullet {},
        }
    }
}
