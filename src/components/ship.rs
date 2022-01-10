use bevy::prelude::Component;

#[derive(Component)]
pub enum RotationDir {
    LEFT,
    RIGHT,
}
#[derive(Component)]
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
#[derive(Component)]
pub struct Engines {
    pub thrust: f32,
    pub spin: f32,
}

#[derive(Component)]
pub struct Thruster;

#[derive(Component)]
pub struct Fuel(pub f32);
