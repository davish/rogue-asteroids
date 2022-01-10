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
        DrawMode::Stroke(StrokeMode {
            color: Color::WHITE,
            options: Default::default(),
        }),
        Transform::default(),
    )
}

impl EntityBundle {
    pub fn new(
        shape: Vec<(f32, f32)>,
        position: RigidBodyPositionComponent,
        velocity: RigidBodyVelocityComponent,
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
                }
                .into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                collider_type: ColliderType::Solid.into(),
                shape: ColliderShape::convex_hull(
                    &(shape
                        .clone()
                        .into_iter()
                        .map(|(x, y)| Point::new(x, y))
                        .collect::<Vec<_>>()),
                )
                .unwrap()
                .into(),
                flags: ActiveEvents::CONTACT_EVENTS.into(),
                ..Default::default()
            },
            physics_sync: RigidBodyPositionSync::Discrete,
            sturdiness: Sturdiness(sturdiness),
        }
    }
}
