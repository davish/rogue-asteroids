use bevy::prelude::*;

use crate::components::{ship::*, types::Player};

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
