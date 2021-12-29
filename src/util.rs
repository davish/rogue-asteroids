use bevy::math::{Vec2, Vec3};

pub fn project2d(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

pub fn from_polar(r: f32, t: f32) -> Vec2 {
    Vec2::new(-t.sin(), t.cos()) * r
}
