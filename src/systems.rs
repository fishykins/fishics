use bevy::prelude::*;
use prima::Intersect;

use crate::{Forces, Mass, RigidBody, Velocity, Collider, BroadPhasePairs};

/// Steps the bodies.
pub fn integration(
    time: Res<Time>,
    mut bodies: Query<(&mut RigidBody, &mut Forces, &mut Velocity, &Mass)>,
) {
    let dt = time.delta_seconds();
    for (mut rb, mut force, mut velocity, mass) in bodies.iter_mut() {
        // Symplectic Euler integration. The order of the next two lines is important!
        velocity.linear += force.collect() * mass.inv() * dt;
        rb.position = rb.position + (velocity.linear * dt);
    }
}

/// Find potential collisions.
pub fn broad_phase(mut bf_pairs: ResMut<BroadPhasePairs>, bodies: Query<(Entity, &RigidBody, &Collider)>) {
    let mut pairs = Vec::new();

    for (entity_a, rb_a, c_a) in bodies.iter() {
        for (entity_b, rb_b, c_b) in bodies.iter() {
            if entity_a == entity_b {
                continue;
            }
            let b_a = c_a.global_aabr(rb_a);
            let b_b = c_b.global_aabr(rb_b);
            if b_a.intersecting(&b_b) {
                if !pairs.contains(&(entity_b, entity_a)) {
                    pairs.push((entity_a, entity_b));
                }
            }
        }
    }
    bf_pairs.pairs = pairs;
}

/// Find actual collisions.
pub fn narrow_phase() {}
