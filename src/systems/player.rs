use bevy::{
    input::touch::TouchPhase,
    prelude::*,
    render::camera::{Camera, CameraPlugin},
};

use crate::components::{
    ship::*,
    types::{Player, Score, ScoreText},
};

pub fn mock_touch(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut touch_events: EventWriter<TouchInput>,
) {
    let window = windows.get_primary().unwrap();
    let touch_phase = if mouse.just_pressed(MouseButton::Left) {
        Some(TouchPhase::Started)
    } else if mouse.just_released(MouseButton::Left) {
        Some(TouchPhase::Ended)
    } else if mouse.pressed(MouseButton::Left) {
        Some(TouchPhase::Moved)
    } else {
        None
    };
    if let (Some(phase), Some(cursor_pos)) = (touch_phase, window.cursor_position()) {
        touch_events.send(TouchInput {
            phase: phase,
            position: cursor_pos,
            force: None,
            id: 0,
        })
    }
}

const DEBOUNCE_DIST: f32 = 10.0;

pub fn player(
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Touches>,
    time: Res<Time>,
    mut query: Query<&mut Controls, With<Player>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();

    let mut controls = query.single_mut();
    controls.last_shot += time.delta_seconds();
    controls.rotate = if keyboard_input.pressed(KeyCode::Left) {
        Some(RotationDir::LEFT)
    } else if keyboard_input.pressed(KeyCode::Right) {
        Some(RotationDir::RIGHT)
    } else {
        None
    };

    controls.thrust = keyboard_input.pressed(KeyCode::Up);
    controls.shoot = keyboard_input.pressed(KeyCode::Space) && controls.last_shot >= 0.25;

    // If any touch input is received, override keyboard.
    for finger in touch_input.iter() {
        if finger.position().x < width * 0.5 {
            let dist = finger.distance();
            if dist.y > DEBOUNCE_DIST {
                controls.thrust = true;
            }
            controls.rotate = if dist.x < -DEBOUNCE_DIST {
                Some(RotationDir::LEFT)
            } else if dist.x > DEBOUNCE_DIST {
                Some(RotationDir::RIGHT)
            } else {
                None
            };
        } else {
            controls.shoot = controls.last_shot >= 0.25;
        }
    }

    if controls.shoot {
        controls.last_shot = 0.0;
    }
}

const TRAIL_DIST: f32 = 200.0;

pub fn camera_tracking(
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_pos = player.single();
    for (mut cam_pos, cam) in camera.iter_mut() {
        // Only move the entity camera, not the UI camera.
        if cam.name == Some(CameraPlugin::CAMERA_2D.to_string()) {
            // Project to 2 dimensions.
            let player_pos2 = Vec2::new(player_pos.translation.x, player_pos.translation.y);
            let cam_pos2 = Vec2::new(cam_pos.translation.x, cam_pos.translation.y);
            let dist = player_pos2.distance(cam_pos2);
            if dist > TRAIL_DIST {
                let new_pos = (cam_pos2 - player_pos2).normalize() * (TRAIL_DIST - dist) + cam_pos2;

                cam_pos.translation.x = new_pos.x;
                cam_pos.translation.y = new_pos.y;
            }
        }
    }
}

pub fn display_score(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    let mut text = text.single_mut();
    text.sections[0].value = format!("Score: {}", score.0);
}
