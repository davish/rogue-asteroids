use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::types::*;

pub fn damage(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut bodies: Query<(Entity, &mut Health), Without<Bullet>>,
    bullets: Query<Entity, With<Bullet>>,
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
                        health.0 -= 20.0;
                        println!("Health for target: {}", health.0);
                    }
                }
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
