use bevy::prelude::Component;

#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct Asteroid;
#[derive(Component)]
pub struct SpawnedAt(pub f64);
#[derive(Component)]
pub struct DespawnAfter(pub f64);
#[derive(Component)]
pub struct Sturdiness(pub f32);
#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct LastAsteroidSpawnTime(pub f64);

#[derive(Component)]
pub struct ScoreText();

#[derive(Default, Debug)]
pub struct Score(pub i32);
