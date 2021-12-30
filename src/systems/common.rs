use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
// use rand::Rng;

use crate::{
    components::{
        chunk::{self, SpawnedChunks},
        types::*,
    },
    entities::asteroid::AsteroidBundle,
    util::project2d,
};

pub fn damage(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut bodies: Query<(Entity, &mut Health), Without<Bullet>>,
    bullets: Query<Entity, With<Bullet>>,
    mut score: ResMut<Score>,
) {
    let is_bullet = |x: &ColliderHandle| bullets.get(x.entity()).is_ok();
    let sort_collision = |a, b: &ColliderHandle| {
        if is_bullet(a) {
            Some((b.entity(), a.entity()))
        } else if is_bullet(b) {
            Some((a.entity(), b.entity()))
        } else {
            None
        }
    };
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(a, b) => {
                if let Some((target, bullet)) = sort_collision(a, b) {
                    commands.entity(bullet).despawn();
                    if let Ok((_entity, mut health)) = bodies.get_mut(target) {
                        health.0 -= 100.0;
                        score.0 += 1;
                        println!("Health for target: {}", health.0);
                    }
                }
                let ea = a.entity();
                let eb = b.entity();
                commands.entity(ea).despawn_recursive();
                commands.entity(eb).despawn_recursive();
            }
            _ => (),
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &SpawnedAt, &DespawnAfter)>,
) {
    for (ent, spawned_at, despawn_after) in query.iter_mut() {
        if time.seconds_since_startup() - spawned_at.0 > despawn_after.0 {
            commands.entity(ent).despawn()
        }
    }
}

pub fn health(mut commands: Commands, mut query: Query<(Entity, &Health)>) {
    for (ent, health) in query.iter_mut() {
        if health.0 <= 0.0 {
            commands.entity(ent).despawn()
        }
    }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    if let Ok(player_pos) = player.single() {
        let surrounding_chunks: Vec<chunk::Chunk> =
            chunk::Chunk::containing_point(&project2d(player_pos.translation)).surrounding_chunks();
        let chunks_to_spawn = surrounding_chunks
            .into_iter()
            .filter(|c| !spawned_chunks.0.contains(c))
            .collect::<Vec<_>>();

        for chunk in chunks_to_spawn {
            AsteroidBundle::spawn_for_chunk(&mut commands, &chunk);
            spawned_chunks.0.insert(chunk);
        }

        // let player2 = project2d(player_pos.translation);
        // let min_dist = asteroids
        //     .iter()
        //     .map(|t| t.translation)
        //     .map(project2d)
        //     .map(|v| v.distance(player2))
        //     .fold(f32::MAX, f32::min);
        // if min_dist != f32::MAX && min_dist > 600.0 {
        //     let linvel: Vec2 = player_vel.linvel.into();
        //     commands.spawn_bundle(AsteroidBundle::new(
        //         (player2 + linvel.normalize() * 300.0).into(),
        //         Default::default(),
        //     ));
        // }
        // for asteroid_pos in asteroids.iter() {
        //     let dist =
        //         project2d(player_pos.translation).distance(project2d(asteroid_pos.translation));

        //     if dist > 600.0 {
        //         println!("rand: {}", rand::thread_rng().gen::<f32>())
        //     }
        // }
    }
}
