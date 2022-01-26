mod components;
mod entities;
mod systems;
mod util;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use components::chunk::{Chunk, SpawnedChunks};
use components::types::{Score, ScoreText};
use wasm_bindgen::prelude::*;

use crate::components::types::{LastAsteroidSpawnTime, Player};
use crate::entities::{asteroid::*, ship::*};
use crate::systems::{common::*, player::*, ship::*};

#[derive(Default, Debug)]
struct LastLog(pub f64);

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .add_system(impulse.system())
        .add_system(player.system())
        .add_system(display_score.system())
        .add_system(weapons.system())
        .add_system(despawn.system())
        .add_system(damage.system())
        .add_system(health.system())
        .add_system(camera_tracking.system())
        // .add_system(spawn_asteroids.system())
        .add_system(mock_touch.system())
        .add_system(log_energy.system())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<LastAsteroidSpawnTime>()
        .init_resource::<SpawnedChunks>()
        .init_resource::<Score>()
        .init_resource::<LastLog>()
        .run();
}

fn log_energy(
    bodies: Query<(&RigidBodyMassProps, &RigidBodyVelocity)>,
    time: Res<Time>,
    mut last_log: ResMut<LastLog>,
) {
    if time.seconds_since_startup() - last_log.0 > 1.0 {
        let mut energy: f32 = 0.0;
        for body in bodies.iter() {
            energy +=
                0.5 * body.0.local_mprops.inv_mass.recip() * body.1.linvel.magnitude_squared();
        }
        println!("Total Kinetic Energy: {}", energy);
        last_log.0 = time.seconds_since_startup();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(Ship::new(Vec2::new(0.0, -215.0).into()))
        .insert(Player {});

    commands.spawn_bundle(AsteroidBundle::new(
        RigidBodyPosition {
            position: (Vec2::new(-200.0, 0.0), 0.0).into(),
            ..Default::default()
        },
        RigidBodyVelocity {
            linvel: Vec2::new(100.0, 0.).into(),
            angvel: 0.0,
        },
        4.0,
    ));

    commands.spawn_bundle(AsteroidBundle::new(
        RigidBodyPosition {
            position: (Vec2::new(200.0, 0.0), 0.0).into(),
            ..Default::default()
        },
        RigidBodyVelocity {
            linvel: Vec2::new(0.0, 0.).into(),
            angvel: 0.0,
        },
        4.0,
    ));
    // AsteroidBundle::spawn_for_chunk(&mut commands, &Chunk::new(0.0, 0.0));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font_size: 32.0,
                    color: Color::WHITE,
                    font: asset_server.load("FiraSans-Bold.ttf"),
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ScoreText {});
}
