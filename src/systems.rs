use bevy::prelude::*;
use prima::{Aabr, Circle, Intersect, Point2, Interact};

use crate::{
    BroadPhasePairs, Collider, ColliderShape, Forces, Manifold, Manifolds, Mass, RigidBody,
    Velocity,
};

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
pub fn broad_phase(
    mut bf_pairs: ResMut<BroadPhasePairs>,
    bodies: Query<(Entity, &RigidBody, &Collider)>,
) {
    let mut pairs = Vec::new();

    // TODO: Quad trees!!!

    let mut combos = bodies.iter_combinations();
    while let Some([(entity_a, rb_a, c_a), (entity_b, rb_b, c_b)]) = combos.next() {
        if c_a.layer & c_b.layer == 0 {
            continue;
        }

        let b_a = c_a.global_aabr(rb_a);
        let b_b = c_b.global_aabr(rb_b);
        if b_a.intersecting(&b_b) {
            // TODO: Better duplicate detection.
            if !pairs.contains(&(entity_b, entity_a)) {
                pairs.push((entity_a, entity_b));
            }
        }
    }
    bf_pairs.pairs = pairs;
}

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

        match a_col.shape {
            ColliderShape::Aabr { half_extents } => {
                let he = Point2::from(half_extents);
                let aabr = Aabr::new(a_rb.position - he, a_rb.position + he);
                match b_col.shape {
                    ColliderShape::Aabr { half_extents } => {
                        let he = Point2::from(half_extents);
                        let aabr_2 = Aabr::new(b_rb.position - he, b_rb.position + he);
                        if aabr.intersecting(&aabr_2) {
                            // Rect-Rect Collision!
                            new_manifolds.push(Manifold::new(*a, *b, aabr.collision(&aabr_2).unwrap()));
                        }
                    }
                    ColliderShape::Circle { radius } => {
                        let circle = Circle::new(b_rb.position, radius);
                        if aabr.intersecting(&circle) {
                            // Rect-Circle Collision!
                            new_manifolds.push(Manifold::new(*a, *b, circle.collision(&aabr).unwrap()));
                        }
                    }
                }
            }
            ColliderShape::Circle { radius } => {
                let circle = Circle::new(a_rb.position, radius);
                match b_col.shape {
                    ColliderShape::Aabr { half_extents } => {
                        let he = Point2::from(half_extents);
                        let aabr = Aabr::new(b_rb.position - he, b_rb.position + he);
                        if aabr.intersecting(&circle) {
                            // Circle-Rect Collision!
                            new_manifolds.push(Manifold::new(*a, *b, circle.collision(&aabr).unwrap()));
                        }
                    }
                    ColliderShape::Circle { radius } => {
                        let circle_2 = Circle::new(b_rb.position, radius);
                        if circle.intersecting(&circle_2) {
                            // Circle-Circle Collision!
                            new_manifolds.push(Manifold::new(*a, *b, circle.collision(&circle_2).unwrap()));
                        }
                    }
                }
            }
        }
    }
    manifolds.set(new_manifolds);
}
