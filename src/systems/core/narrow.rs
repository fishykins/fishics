use bevy::prelude::*;
use prima::Interact;

use crate::{pipeline::{Manifolds, BroadPhasePairs, Manifold}, components::{RigidBody, Collider}};

/// Find actual collisions.
pub fn narrow_phase(
    mut manifolds: ResMut<Manifolds>,
    pairs: Res<BroadPhasePairs>,
    bodies: Query<&RigidBody>,
    colliders: Query<&Collider>,
) {
    let mut new_manifolds = Vec::new();

    for (a, b) in pairs.pairs.iter() {
        let a_rb = bodies.get(*a).unwrap();
        let b_rb = bodies.get(*b).unwrap();
        let a_col = colliders.get(*a).unwrap();
        let b_col = colliders.get(*b).unwrap();

        let a_shape = a_col.shape.wrap(a_rb.position());
        let b_shape = b_col.shape.wrap(b_rb.position());

        if let Some(collision) = a_shape.collision(&b_shape) {
            //println!("{:?}", collision);
            new_manifolds.push(Manifold::new(*a, *b, collision));
        }
    }
    manifolds.set(new_manifolds);
}
