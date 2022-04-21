use bevy::prelude::*;
use prima::{Dot, Interact, Intersect, Vector};

use crate::{
    BroadPhasePairs, Collider, Forces, Manifold, Manifolds, Mass, PhysicsMaterial, RigidBody,
    Velocity,
};

/// Steps the bodies.
pub fn integration(
    time: Res<Time>,
    mut bodies: Query<(&mut RigidBody, &mut Forces, &mut Velocity, &Mass)>,
) {
    let dt = time.delta_seconds();
    for (mut rb, mut force, mut velocity, mass) in bodies.iter_mut() {
        if mass.raw() == 0.0 {
            continue;
        }
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
            pairs.push((entity_a, entity_b));
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
            //println!("{:?}", collision);
            new_manifolds.push(Manifold::new(*a, *b, collision));
        }
    }
    manifolds.set(new_manifolds);
}

pub fn impulse_resolution(
    time: Res<Time>,
    manifolds: Res<Manifolds>,
    mut vel: Query<&mut Velocity>,
    mat: Query<&PhysicsMaterial>,
    mass: Query<&Mass>,
) {
    for m in manifolds.iter() {
        let collision = m.collision.clone();
        let a_vel = vel.get(m.a).unwrap();
        let b_vel = vel.get(m.b).unwrap();
        let a_mat = mat.get(m.a).unwrap();
        let b_mat = mat.get(m.b).unwrap();
        let a_mass = mass.get(m.a).unwrap();
        let b_mass = mass.get(m.b).unwrap();
        let a_mass_inv = a_mass.inv();
        let b_mass_inv = b_mass.inv();

        let initial_magnitude_squared = a_vel.linear_mag_squared() + b_vel.linear_mag_squared();

        // Calculate relative velocity
        let rv: Vector<f32> = b_vel.linear() - a_vel.linear();

        // Calc. relative velocity in terms of the normal direction
        let velocity_along_normal = rv.dot(&collision.normal);

        // Do not resolve if velocities are separating
        if velocity_along_normal > 0.0 {
            println!("resolving");
            continue;
        }

        // min restitution value.
        let e = a_mat.restitution.min(b_mat.restitution);
        let dt = time.delta_seconds();

        // Calc impulse scalar
        let j = (-(1.0 + e) * velocity_along_normal) / (a_mass_inv + b_mass_inv);

        // Apply impulse
        let mass_sum = a_mass.raw() + b_mass.raw();
        let ratio_a = a_mass.raw() / mass_sum;
        let ratio_b = b_mass.raw() / mass_sum;
        let impulse = collision.normal * j;
        let mut a_v = a_vel.linear() - impulse * ratio_b * dt;
        let mut b_v = b_vel.linear() + impulse * ratio_a * dt;

        //? End of primary resolution- moving on to apply friction.

        // Calculate the new relative velocity.
        let rv: Vector<f32> = b_v - a_v;

        // Solve for tangent vector
        let t: Vector<f32> = rv - (collision.normal * rv.dot(&collision.normal));
        let t = t.normalize();

        // Solve for magnitude to apply along friction line
        let jt = -(rv.dot(&t)) / (a_mass_inv + b_mass_inv);

        if jt > 0.0 {
            println!(
                "rv: {:?}, van: {}, j: {}, t: {:?}, jt: {}",
                rv, velocity_along_normal, j, t, jt
            );

            // PythagoreanSolve = A^2 + B^2 = C^2, solving for C given A and B
            // Use to approximate mu given friction coefficients of each body
            let mu = pythag_solver(a_mat.static_friction, b_mat.static_friction);

            // Clamp magnitude of friction and create impluse vector
            let friction_impulse = if jt.abs() < mu * jt {
                t * jt
            } else {
                t * -j * pythag_solver(a_mat.dynamic_friction, b_mat.dynamic_friction)
            };

            a_v -= friction_impulse * a_mass_inv * dt;
            b_v += friction_impulse * b_mass_inv * dt;
        }

        // Lets avoid drawing Newton from his grave and prevent the collision from generating more energy than it already has.
        let resultant_magnitude_squared = pythag_sqr(a_v.x, a_v.y) + pythag_sqr(b_v.x, b_v.y);
        let ratio = (initial_magnitude_squared / resultant_magnitude_squared).min(1.0);

        a_v = a_v * ratio;
        b_v = b_v * ratio;

        vel.get_mut(m.a).unwrap().set_linear(a_v);
        vel.get_mut(m.b).unwrap().set_linear(b_v);
    }
}

fn pythag_solver(a: f32, b: f32) -> f32 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn pythag_sqr(a: f32, b: f32) -> f32 {
    a.powi(2) + b.powi(2)
}
