use bevy::{ecs::query::QueryEntityError, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        chunk::{self, SpawnedChunks},
        types::*,
    },
    entities::asteroid::AsteroidBundle,
    util::{project2d, STURDINESS_CONSTANT},
};

fn process_collision(
    bodies: &mut Query<(
        Entity,
        &mut Sturdiness,
        &RigidBodyMassProps,
        &RigidBodyVelocity,
        &ColliderMaterial,
    )>,
    a: Entity,
    b: Entity,
) -> Result<(), QueryEntityError> {
    let mut get_components = |e: Entity| -> Result<(f32, Vec2, f32), QueryEntityError> {
        let obj = bodies.get_mut(e)?;
        let m = obj.2.local_mprops.inv_mass.recip();
        let v: Vec2 = obj.3.linvel.into();
        Ok((m, v, obj.4.restitution))
    };

    let (ma, va, rsta) = get_components(a)?;
    let (mb, vb, rstb) = get_components(b)?;
    let relv2 = (va - vb).length_squared();
    let restitution: f32 = (rsta + rstb) / 2.; // TODO: support other CoeffecientCombineRules beyond Average.

    let e_a = 0.5 * mb * relv2;
    let e_b = 0.5 * ma * relv2;

    let rel_e_a = e_a * (1.0 - restitution.powf(2.0));
    let rel_e_b = e_b * (1.0 - restitution.powf(2.0));

    let mut step_sturdiness = |entity, energy: f32| -> Result<(), QueryEntityError> {
        let mut sturdiness = bodies.get_mut(entity)?.1;
        sturdiness.0 = sturdiness.0 - STURDINESS_CONSTANT * energy / sturdiness.0;
        Ok(())
    };
    let _ = step_sturdiness(a, rel_e_a)?;
    let _ = step_sturdiness(b, rel_e_b)?;

    Ok(())
}

pub fn damage(
    mut contact_events: EventReader<ContactEvent>,
    mut bodies: Query<(
        Entity,
        &mut Sturdiness,
        &RigidBodyMassProps,
        &RigidBodyVelocity,
        &ColliderMaterial,
    )>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(a, b) => {
                let _ = process_collision(&mut bodies, a.entity(), b.entity());
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

pub fn health(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &RigidBodyPosition,
        &RigidBodyVelocity,
        &RigidBodyMassProps,
        &Sturdiness,
        Option<&Asteroid>,
    )>,
) {
    for (ent, pos, vel, mass, sturdiness, asteroid) in query.iter_mut() {
        if sturdiness.0 <= 0.0 {
            commands.entity(ent).despawn();
            if let Some(Asteroid(scale)) = asteroid {
                if *scale > 2. {
                    for daughter in
                        AsteroidBundle::generate_daughters(pos, vel, mass, *scale, sturdiness.0)
                    {
                        commands.spawn_bundle(daughter);
                    }
                }
            }
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
    }
}
