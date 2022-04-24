use bevy::ecs::system::Resource;
use bevy_inspector_egui::Inspectable;
use prima::Dot;

use crate::pipeline::{ImpulseObject, ImpulseResult, Manifold};

/// A trait that allows the struct to be used to solve collisions. While the classic 'resolver' supplied in the crate does not
/// make any use of 'self', it is there if you want to use your own custom collision resolution algorithm.
pub trait ImpulseResolver: Default + Resource {
    /// Tells the resolver a new round of collision resolution is starting.
    fn tick(&mut self);
    /// Resolves the collision between two bodies.
    fn resolve(&mut self, manifold: Manifold, a: ImpulseObject, b: ImpulseObject) -> (ImpulseResult, ImpulseResult);
}

/// Calculates the resulting velocities of two objects after a collision.
/// This is technically a Resource and not a system, but it is used in the same way so it gets to hang out with the rest of the systems.
/// to the physics loop it gets to hang out in the pipeline.
#[derive(Default, Debug, Clone, Copy, Inspectable)]
pub struct ClassicImpulseResolver {
    #[inspectable(read_only)]
    ticks: u32,
    #[inspectable(read_only)]
    collisions: u32,
}

impl ImpulseResolver for ClassicImpulseResolver {
    fn tick(&mut self) {
        if let Some(t) = self.ticks.checked_add(1) {
            self.ticks = t;
        } else {
            self.ticks = 0;
        }
    }

    fn resolve(
        &mut self,
        m: Manifold,
        a: ImpulseObject,
        b: ImpulseObject,
    ) -> (ImpulseResult, ImpulseResult) {
        let mut r1: ImpulseResult = a.into();
        let mut r2: ImpulseResult = b.into();
    
        if m.f() <= 0. {
            return (r1, r2);
        }

        self.collisions += 1;
    
        let rv_n = (a.v - b.v).dot(&m.n);
        let e = 1.0;
        let j = (-(1.0 + e) * rv_n) / (a.i + b.i);
        r1.v = a.v + (m.n * a.i * j);
        r2.v = b.v - (m.n * b.i * j);
    
        println!("boop");
        (r1, r2)
    }
}