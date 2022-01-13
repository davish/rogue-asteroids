use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::types::*;

#[derive(Bundle)]
pub struct EntityBundle {
    #[bundle]
    geometry: ShapeBundle,
    #[bundle]
    body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    physics_sync: RigidBodyPositionSync,
    sturdiness: Sturdiness,
}

pub fn build_geometry(shape: &[(f32, f32)]) -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Polygon {
            points: shape
                .clone()
                .into_iter()
                .map(|(x, y)| Vec2::new(*x, *y))
                .collect(),
            closed: true,
        },
        ShapeColors {
            main: Color::WHITE,
            outline: Color::WHITE,
        },
        DrawMode::Stroke(StrokeOptions::default()),
        Transform::default(),
    )
}

impl EntityBundle {
    pub fn new(
        shape: Vec<(f32, f32)>,
        position: RigidBodyPosition,
        velocity: RigidBodyVelocity,
        sturdiness: f32,
    ) -> Self {
        EntityBundle {
            geometry: build_geometry(&shape),
            body: RigidBodyBundle {
                position,
                velocity,
                forces: RigidBodyForces {
                    gravity_scale: 0.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: ColliderBundle {
                collider_type: ColliderType::Solid,
                shape: ColliderShape::convex_hull(
                    &(shape
                        .clone()
                        .into_iter()
                        .map(|(x, y)| Point::new(x, y))
                        .collect::<Vec<_>>()),
                )
                .unwrap(),
                flags: ActiveEvents::CONTACT_EVENTS.into(),
                material: ColliderMaterial {
                    restitution: 0.9,
                    ..Default::default()
                },
                ..Default::default()
            },
            physics_sync: RigidBodyPositionSync::Discrete,
            sturdiness: Sturdiness(sturdiness),
        }
    }
}
