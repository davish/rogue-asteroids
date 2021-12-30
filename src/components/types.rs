pub struct Bullet;
pub struct Asteroid;
pub struct SpawnedAt(pub f64);
pub struct DespawnAfter(pub f64);
pub struct Health(pub f32);
pub struct Player;

#[derive(Default)]
pub struct LastAsteroidSpawnTime(pub f64);

pub struct ScoreText();

#[derive(Default, Debug)]
pub struct Score(pub i32);
