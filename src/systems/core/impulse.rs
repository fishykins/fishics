use bevy::prelude::*;

use crate::{
    components::{Mass, RigidBody, Velocity},
    pipeline::{generate_impulse_pair, Manifolds},
};

use super::resolve_impulse;

pub fn impulse_resolution (
    time: Res<Time>,
    manifolds: Res<Manifolds>,
    rbq: Query<&RigidBody>,
    mq: Query<&Mass>,
    mut vq: Query<&mut Velocity>,
) {
    let dt = time.delta_seconds();

    for manifold in manifolds.iter() {
        // Collect impulse data.
        let (a, b) = generate_impulse_pair(manifold, &mut vq, &mq, &rbq);

        // Calculate the initial force of the collision.
        let initial_force = a.m * a.v.magnitude() + b.m * b.v.magnitude();

        // Send impulse data to the collision resolution function.
        let (a, b) = resolve_impulse(manifold.with_initial_force(initial_force), a, b, dt);

        // Apply impulses!
        let mut va = vq.get_mut(manifold.a).unwrap();
        va.set_linear(a.v);
        va.set_angular(a.r);
        let mut vb = vq.get_mut(manifold.b).unwrap();
        vb.set_linear(b.v);
        vb.set_angular(b.r);

        //std::process::exit(6002);
    }
}
