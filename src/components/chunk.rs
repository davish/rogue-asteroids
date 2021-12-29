use bevy::{math::Vec2, utils::HashSet};
use itertools::*;
use rand::{prelude::ThreadRng, Rng};

const CHUNK_SIZE: f32 = 600.0;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Chunk(i32, i32);

impl Chunk {
    pub fn new(x: f32, y: f32) -> Self {
        Chunk(x.round() as i32, y.round() as i32)
    }
    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }

    pub fn top_right(&self) -> Vec2 {
        self.bottom_left() + Vec2::new(CHUNK_SIZE, CHUNK_SIZE)
    }

    pub fn random_point_inside(&self, rng: &mut ThreadRng) -> Vec2 {
        let lower = self.bottom_left();
        let upper = self.top_right();
        Vec2::new(
            rng.gen_range(lower.x..upper.x),
            rng.gen_range(lower.y..upper.y),
        )
    }

    pub fn containing_point(pt: &Vec2) -> Self {
        let closest_boundary = |x: f32| x - (x % CHUNK_SIZE);
        Self::new(closest_boundary(pt.x), closest_boundary(pt.y))
    }

    pub fn surrounding_chunks(&self) -> Vec<Self> {
        let as_vec = Vec2::new(self.0 as f32, self.1 as f32);
        [-2.0, -1.0, 0.0, 1.0]
            .iter()
            .cartesian_product([-2.0, -1.0, 0.0, 1.0])
            .map(|(dx, dy)| (dx * CHUNK_SIZE, dy * CHUNK_SIZE))
            .map(|(dx, dy)| Chunk::new(as_vec.x + dx, as_vec.y + dy))
            .collect::<Vec<_>>()
    }
}

impl From<Vec2> for Chunk {
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

/// HashSet of chunks which have been spawned.
/// A HashSet is defined by
#[derive(Default)]
pub struct SpawnedChunks(pub HashSet<Chunk>);
