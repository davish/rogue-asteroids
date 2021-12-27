use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct Health(pub f32);

#[derive(Bundle)]
pub struct EntityBundle {
    #[bundle]
    geometry: ShapeBundle,
    #[bundle]
    body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    physics_sync: RigidBodyPositionSync,
    health: Health,
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
                ..Default::default()
            },
            physics_sync: RigidBodyPositionSync::Discrete,
            health: Health(100.0),
        }
    }
}
pub struct SpawnedAt(pub f64);
pub struct DespawnAfter(pub f64);

pub fn despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &SpawnedAt, &DespawnAfter)>,
) {
    for (ent, spawned_at, despawn_after) in query.iter_mut() {
        if time.seconds_since_startup() - spawned_at.0 > despawn_after.0 {
            commands.entity(ent).despawn()
        }
    }
}
