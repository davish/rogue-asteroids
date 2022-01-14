use bevy::math::{Mat2, Vec2, Vec3};

pub const STURDINESS_CONSTANT: f32 = 1.0 / 500.0;

pub fn project2d(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

pub fn from_polar(r: f32, t: f32) -> Vec2 {
    Vec2::new(-t.sin(), t.cos()) * r
}

pub fn rotate(v: Vec2, t: f32) -> Vec2 {
    let rotation = Mat2::from_cols(Vec2::new(t.cos(), -t.sin()), Vec2::new(t.sin(), t.cos()));
    rotation * v
}

pub fn add_along(along: Vec2, vec: Vec2, add: Vec2) -> Vec2 {
    vec + rotate(add, along.angle_between(Vec2::new(1., 0.)))
}
