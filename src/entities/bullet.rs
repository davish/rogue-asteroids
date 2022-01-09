use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::components::types::*;

use super::entity::EntityBundle;

lazy_static! {
    static ref BULLET_SHAPE: Vec<(f32, f32)> =
        vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];
}

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
        let launch_vel = Vec2::new(-angle.sin(), angle.cos()) * 500.0 + vel.linvel.into();
        let ship_position: Vec2 = pos.position.translation.into();
        let launch_position: Vec2 = ship_position + Vec2::new(-angle.sin(), angle.cos()) * 20.0;
        Self {
            base: EntityBundle::new(
                BULLET_SHAPE.clone(),
                launch_position.into(),
                RigidBodyVelocity {
                    linvel: launch_vel.into(),
                    angvel: 0.0,
                },
                1.0,
            ),
            launch_time: SpawnedAt(launch_time),
            despawn_after: DespawnAfter(3.0),
            bullet: Bullet {},
        }
    }
}
