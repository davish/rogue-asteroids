use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use super::bullet::BulletBundle;
use super::entity::build_geometry;
use super::entity::EntityBundle;

lazy_static! {
    static ref SPACESHIP_SHAPE: Vec<(f32, f32)> = vec![(-6.0, -10.0), (0.0, 14.0), (6.0, -10.0),];
    static ref EXHAUST_SHAPE: Vec<(f32, f32)> = vec![(-3.0, -12.0), (0.0, -20.0), (3.0, -12.0)];
}

pub enum RotationDir {
    LEFT,
    RIGHT,
}
pub struct Controls {
    pub thrust: bool,
    pub rotate: Option<RotationDir>,
    pub shoot: bool,
    pub last_shot: f32,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            thrust: false,
            rotate: None,
            shoot: false,
            last_shot: 0.0,
        }
    }
}
pub struct Engines {
    pub thrust: f32,
    pub spin: f32,
}

pub struct Thruster;

#[derive(Bundle)]
pub struct ThrusterBundle {
    #[bundle]
    base: ShapeBundle,

    thruster: Thruster,
}
pub struct Fuel(pub f32);

pub struct Player;

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

pub fn player_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Controls, With<Player>>,
) {
    if let Ok(mut controls) = query.single_mut() {
        controls.rotate = if keyboard_input.pressed(KeyCode::Left) {
            Some(RotationDir::LEFT)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Some(RotationDir::RIGHT)
        } else {
            None
        };

        controls.thrust = keyboard_input.pressed(KeyCode::Up);

        controls.last_shot += time.delta_seconds();
        controls.shoot = keyboard_input.pressed(KeyCode::Space) && controls.last_shot >= 0.25;
        if controls.shoot {
            controls.last_shot = 0.0;
        }
    }
}

pub fn weapons_system(
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

pub fn impulse_system(
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
