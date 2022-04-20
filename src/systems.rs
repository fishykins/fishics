use bevy::prelude::*;
use prima::{Dot, Interact, Intersect, Vector2};

use crate::{
    BroadPhasePairs, Collider, Forces, Manifold, Manifolds, Mass, PhysicsMaterial,
    RigidBody, Velocity,
};

/// Steps the bodies.
pub fn integration(
    time: Res<Time>,
    mut bodies: Query<(&mut RigidBody, &mut Forces, &mut Velocity, &Mass)>,
) {
    let dt = time.delta_seconds();
    for (mut rb, mut force, mut velocity, mass) in bodies.iter_mut() {
        // Symplectic Euler integration. The order of the next two lines is important!
        velocity.add_linear(force.collect() * mass.inv() * dt);
        rb.translate(velocity.linear() * dt);
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

        let a_shape = a_col.shape.wrap(a_rb.position());
        let b_shape = b_col.shape.wrap(b_rb.position());

        if let Some(collision) = a_shape.collision(&b_shape) {
            new_manifolds.push(Manifold::new(
                *a,
                *b,
                collision,
            ));
        }
    }
    manifolds.set(new_manifolds);
}

pub fn impulse_resolution(
    manifolds: Res<Manifolds>,
    mut vel: Query<&mut Velocity>,
    mat: Query<&PhysicsMaterial>,
    mass: Query<&Mass>,
) {
    for m in manifolds.iter() {
        let collision = m.collision.clone();
        let a_vel = vel.get(m.a).unwrap();
        let b_vel = vel.get(m.b).unwrap();
        let a_restitution = mat.get(m.a).unwrap().restitution;
        let b_restitution = mat.get(m.b).unwrap().restitution;
        let a_mass = mass.get(m.a).unwrap().inv();
        let b_mass = mass.get(m.b).unwrap().inv();
        let e = a_restitution.min(b_restitution);

        // Calculate relative velocity
        let rv: Vector2<f32> = b_vel.linear() - a_vel.linear();

        // Calc. relative velocity in terms of the normal direction
        let velocity_along_normal = rv.dot(&collision.normal);

        // Do not resolve if velocities are separating
        if velocity_along_normal > 0.0 {
            return;
        }
        
        // Calc impulse scalar
        let j = -(1.0 + e) * velocity_along_normal;
        let j = j / a_mass + j / b_mass;
        
        // Apply impulse
        let impulse = collision.normal * j;

        // Distribute according to mass
        let mass_sum = a_mass + b_mass;
        let ratio_a = a_mass / mass_sum;
        let ratio_b = b_mass / mass_sum;

        vel.get_mut(m.a).unwrap().sub_linear(impulse * ratio_a);
        vel.get_mut(m.b).unwrap().add_linear(impulse * ratio_b);

        println!("BOOP");
    }
}