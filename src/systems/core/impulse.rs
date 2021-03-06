use bevy::prelude::*;
use prima::prelude::*;

use crate::{
    components::{Mass, RigidBody, Velocity, PhysicsMaterial},
    pipeline::{generate_impulse_pair, Manifolds},
};

use super::ImpulseResolver;

pub fn impulse_resolution<F>(
    mut resolver: ResMut<F>,
    mut vq: Query<&mut Velocity>,
    mut rbq: Query<&mut RigidBody>,
    manifolds: Res<Manifolds>,
    materials: Res<Assets<PhysicsMaterial>>,
    mq: Query<&Mass>,
    mat_handles: Query<&Handle<PhysicsMaterial>>,
) where
    F: ImpulseResolver,
{
    resolver.tick();
    
    for manifold in manifolds.iter() {
        // Collect impulse data.
        let (a, b) = generate_impulse_pair(&mut vq, &mut rbq, manifold, &mq, &materials, &mat_handles);

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

        let mut rba = rbq.get_mut(manifold.a).unwrap();
        rba.position += Vec2::new(a.t.x, a.t.y);
        let mut rbb = rbq.get_mut(manifold.b).unwrap();
        rbb.position += Vec2::new(b.t.x, b.t.y);
    }
}
