use bevy::{
    prelude::*,
    render::{camera::Camera, render_graph::base::camera},
};

use crate::components::{
    ship::*,
    types::{Player, Score, ScoreText},
};

pub fn player(
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

const TRAIL_DIST: f32 = 200.0;

pub fn camera_tracking(
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(player_pos) = player.single() {
        for (mut cam_pos, cam) in camera.iter_mut() {
            // Only move the entity camera, not the UI camera.
            if cam.name == Some(camera::CAMERA_2D.to_string()) {
                // Project to 2 dimensions.
                let player_pos2 = Vec2::new(player_pos.translation.x, player_pos.translation.y);
                let cam_pos2 = Vec2::new(cam_pos.translation.x, cam_pos.translation.y);
                let dist = player_pos2.distance(cam_pos2);
                if dist > TRAIL_DIST {
                    let new_pos =
                        (cam_pos2 - player_pos2).normalize() * (TRAIL_DIST - dist) + cam_pos2;

                    cam_pos.translation.x = new_pos.x;
                    cam_pos.translation.y = new_pos.y;
                }
            }
        }
    }
}

pub fn display_score(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = text.single_mut() {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}
