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

pub struct Fuel(pub f32);
