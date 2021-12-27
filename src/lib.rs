mod components;
mod entities;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use wasm_bindgen::prelude::*;

use crate::components::types::{Asteroid, Player};
use crate::entities::{asteroid::*, ship::*};
use crate::systems::{common::*, player::*, ship::*};

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
        .add_system(weapons.system())
        .add_system(despawn.system())
        .add_system(damage.system())
        .add_system(health.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(Ship::new(Vec2::new(0.0, -215.0).into()))
        .insert(Player {});

    commands
        .spawn_bundle(new_asteroid_bundle())
        .insert(Asteroid {});

    commands.spawn_bundle(TextBundle {
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
    });
}
