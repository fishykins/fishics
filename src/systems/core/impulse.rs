use bevy::prelude::*;

use crate::{
    components::{Mass, RigidBody, Velocity},
    pipeline::{generate_impulse_pair, Manifolds},
};

use super::ImpulseResolver;

pub fn impulse_resolution<F>(
    mut resolver: ResMut<F>,
    manifolds: Res<Manifolds>,
    rbq: Query<&RigidBody>,
    mq: Query<&Mass>,
    mut vq: Query<&mut Velocity>,
) where
    F: ImpulseResolver,
{
    resolver.tick();
    
    for manifold in manifolds.iter() {
        // Collect impulse data.
        let (a, b) = generate_impulse_pair(manifold, &mut vq, &mq, &rbq);

        // Calculate the initial force of the collision.
        let initial_force = a.m * a.v.magnitude() + b.m * b.v.magnitude();

        // Send impulse data to the collision resolution function.
        let (a, b) = resolver.resolve(manifold.with_initial_force(initial_force), a, b);

        // Apply impulses!
        let mut va = vq.get_mut(manifold.a).unwrap();
        va.set_linear(a.v);
        va.set_angular(a.r);
        let mut vb = vq.get_mut(manifold.b).unwrap();
        vb.set_linear(b.v);
        vb.set_angular(b.r);
    }
}
