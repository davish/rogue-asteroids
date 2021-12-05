use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref SPACESHIP_SHAPE: Vec<(f32, f32)> = vec![(-6.0, -10.0), (0.0, 14.0), (6.0, -10.0),];
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

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .add_system(ship_control_system.system())
        .run();
}

struct Spaceship {
    thrust: f32,
    spin: f32,
}

fn spawn_spaceship(commands: &mut Commands, ship: Spaceship, pos: Vec2) {
    let ship_shape = shapes::Polygon {
        points: SPACESHIP_SHAPE
            .clone()
            .into_iter()
            .map(|(x, y)| Vec2::new(x, y))
            .collect(),
        closed: true,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &ship_shape,
            ShapeColors {
                main: Color::WHITE,
                outline: Color::WHITE,
            },
            DrawMode::Stroke(StrokeOptions::default()),
            Transform::default(),
        ))
        .insert(ship)
        .insert_bundle(RigidBodyBundle {
            position: pos.into(),
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Solid,
            shape: ColliderShape::convex_hull(
                &(SPACESHIP_SHAPE
                    .clone()
                    .into_iter()
                    .map(|(x, y)| Point::new(x, y))
                    .collect::<Vec<_>>()),
            )
            .unwrap(),
            ..Default::default()
        })
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete);
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_spaceship(
        &mut commands,
        Spaceship {
            thrust: 50000.0,
            spin: 5.0,
        },
        Vec2::new(0.0, -215.0),
    );

    let asteroid_points = ASTEROID_SHAPE
        .clone()
        .into_iter()
        .map(|p| (p.0 * 4.0, p.1 * 4.0))
        .collect::<Vec<(f32, f32)>>();

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Polygon {
                points: asteroid_points
                    .iter()
                    .cloned()
                    .map(|(x, y)| Vec2::new(x, y))
                    .collect(),
                closed: true,
            },
            ShapeColors {
                main: Color::WHITE,
                outline: Color::WHITE,
            },
            DrawMode::Stroke(StrokeOptions::default()),
            Transform::default(),
        ))
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 0.0).into(),
            velocity: RigidBodyVelocity {
                angvel: 1.0,
                ..Default::default()
            },
            forces: RigidBodyForces {
                gravity_scale: 0.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Solid,
            shape: ColliderShape::convex_hull(
                &(asteroid_points
                    .iter()
                    .cloned()
                    .map(|(x, y)| Point::new(x, y))
                    .collect::<Vec<_>>()),
            )
            .unwrap(),
            ..Default::default()
        })
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete);
}

fn ship_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Spaceship,
        &RigidBodyPosition,
        &mut RigidBodyVelocity,
        &mut RigidBodyForces,
    )>,
) {
    if let Ok((ship, pos, mut vel, mut forces)) = query.single_mut() {
        vel.angvel = if keyboard_input.pressed(KeyCode::Left) {
            ship.spin
        } else if keyboard_input.pressed(KeyCode::Right) {
            -ship.spin
        } else {
            0.0
        };

        forces.force = (if keyboard_input.pressed(KeyCode::Up) {
            let angle = pos.position.rotation.angle();
            Vec2::new(-angle.sin(), angle.cos())
        } else {
            Vec2::default()
        } * ship.thrust)
            .into();
    }
}
