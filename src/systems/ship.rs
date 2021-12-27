use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::ship::*;
use crate::entities::{bullet::BulletBundle, entity::build_geometry, ship::EXHAUST_SHAPE};

pub fn weapons(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Controls, &RigidBodyPosition, &RigidBodyVelocity)>,
) {
    for (controls, pos, vel) in query.iter_mut() {
        if controls.shoot {
            commands.spawn_bundle(BulletBundle::launch_from(
                pos,
                vel,
                time.seconds_since_startup(),
            ));
        }
    }
}

pub fn impulse(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Controls,
        &Engines,
        &RigidBodyPosition,
        &mut RigidBodyVelocity,
        &mut RigidBodyForces,
        Option<&Children>,
    )>,
    thruster_query: Query<Entity, With<Thruster>>,
) {
    for (entity, controls, engines, pos, mut vel, mut forces, children) in query.iter_mut() {
        vel.angvel = match controls.rotate {
            Some(RotationDir::LEFT) => engines.spin,
            Some(RotationDir::RIGHT) => -engines.spin,
            None => 0.0,
        };

        match children.map_or(None, |c| {
            c.iter()
                .find(|child| thruster_query.get(*child.clone()).is_ok())
        }) {
            Some(thruster) => {
                if !controls.thrust {
                    commands.entity(thruster.clone()).despawn()
                }
            }
            None => {
                if controls.thrust {
                    let thruster = commands
                        .spawn_bundle(build_geometry(&EXHAUST_SHAPE))
                        .insert(Thruster {})
                        .id();
                    commands.entity(entity).push_children(&[thruster]);
                }
            }
        };

        forces.force = (if controls.thrust {
            let angle = pos.position.rotation.angle();
            Vec2::new(-angle.sin(), angle.cos())
        } else {
            Vec2::default()
        } * engines.thrust)
            .into();
    }
}
