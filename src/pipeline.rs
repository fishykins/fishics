use crate::components::{Mass, RigidBody, Velocity, PhysicsMaterial};
use bevy::prelude::*;
use prima::prelude::*;
#[derive(Debug, Clone)]
pub struct BroadPhasePairs {
    pub pairs: Vec<(Entity, Entity)>,
}

#[derive(Debug, Clone)]
pub struct Manifolds(Vec<Manifold>);
#[derive(Debug, Clone)]
pub struct Manifold {
    pub a: Entity,
    pub b: Entity,
    pub n: Vector<f32>,
    pub p: f32,
    pub f: Option<f32>,
    pub c: Point<f32>,
}

// Impulse pipeline helper-structs.

/// Stores associated data for a collision object.
#[derive(Debug, Clone, Copy)]
pub struct ImpulseObject {
    /// mass
    pub m: f32,
    /// inverse mass
    pub i: f32,
    /// initial velocity
    pub v: Vector<f32>,
    /// initial angular velocity
    pub r: f32,
    /// 'center of mass', or position
    pub c: Point<f32>,
    /// inverse moment of inertia
    pub mi: f32,
    /// Coefficient of restitution
    pub cr: f32,
}

/// Result of a collision on an object.
#[derive(Debug, Clone, Copy)]
pub struct ImpulseResult {
    /// new linear velocity
    pub v: Vector<f32>,
    /// new angular velocity
    pub r: f32,
}

pub fn generate_impulse_pair(
    v: &mut Query<&mut Velocity>,
    manifold: &Manifold,
    m: &Query<&Mass>,
    rb: &Query<&RigidBody>,
    mats: &Res<Assets<PhysicsMaterial>>,
    mat_handles: &Query<&Handle<PhysicsMaterial>>,
) -> (ImpulseObject, ImpulseObject) {
    let vel_1 = v.get(manifold.a).ok();
    let vel_2 = v.get(manifold.b).ok();
    let mass_1 = m.get(manifold.a).ok();
    let mass_2 = m.get(manifold.b).ok();

    let mi_1 = 1.0;
    let mi_2 = 1.0;
    let mut cr_1 = 1.0;
    let mut cr_2 = 1.0;

    if let Some(handle) = mat_handles.get(manifold.a).ok() {
        if let Some(mat) = mats.get(handle) {
            cr_1 = mat.restitution;
        }
    }
    if let Some(handle) = mat_handles.get(manifold.b).ok() {
        if let Some(mat) = mats.get(handle) {
            cr_2 = mat.restitution;
        }
    }


    let v1 = if let Some(vel_1) = vel_1 {
        vel_1.linear()
    } else {
        Vector::zero()
    };
    let v2 = if let Some(vel_2) = vel_2 {
        vel_2.linear()
    } else {
        Vector::zero()
    };
    let a1 = if let Some(vel_1) = vel_1 {
        vel_1.angular()
    } else {
        0.0
    };
    let a2 = if let Some(vel_2) = vel_2 {
        vel_2.angular()
    } else {
        0.0
    };
    let m1 = if let Some(mass_1) = mass_1 {
        mass_1.raw()
    } else {
        0.0
    };
    let m2 = if let Some(mass_2) = mass_2 {
        mass_2.raw()
    } else {
        0.0
    };
    let i1 = if let Some(mass_1) = mass_1 {
        mass_1.inv()
    } else {
        0.0
    };
    let i2 = if let Some(mass_2) = mass_2 {
        mass_2.inv()
    } else {
        0.0
    };
    let com1 = rb.get(manifold.a).unwrap().position;
    let com2 = rb.get(manifold.b).unwrap().position;

    let a: ImpulseObject = ImpulseObject {
        m: m1,
        i: i1,
        v: v1,
        r: a1,
        c: Point::new(com1.x, com1.y),
        cr: cr_1,
        mi: mi_1,
    };

    let b: ImpulseObject = ImpulseObject {
        m: m2,
        i: i2,
        v: v2,
        r: a2,
        c: Point::new(com2.x, com2.y),
        cr: cr_2,
        mi: mi_2,
    };

    (a, b)
}

impl ImpulseResult {
    pub fn new(v: Vector<f32>, r: f32) -> Self {
        ImpulseResult { v, r }
    }

    pub fn none() -> (Self, Self) {
        (
            ImpulseResult {
                v: Vector::zero(),
                r: 0.0,
            },
            ImpulseResult {
                v: Vector::zero(),
                r: 0.0,
            },
        )
    }
}

impl Into<ImpulseResult> for ImpulseObject {
    fn into(self) -> ImpulseResult {
        ImpulseResult {
            v: self.v,
            r: self.r,
        }
    }
}

impl BroadPhasePairs {
    pub fn new() -> Self {
        BroadPhasePairs { pairs: Vec::new() }
    }
}

impl Manifold {
    pub fn new(a: Entity, b: Entity, collision: Collision<f32>) -> Self {
        let n = collision.normal.normalize();
        let p = collision.depth;
        Self {
            a,
            b,
            n,
            p,
            f: None,
            c: collision.point,
        }
    }

    pub fn with_initial_force(&self, f: f32) -> Self {
        let mut new = self.clone();
        new.f = Some(f);
        new
    }

    pub fn f(&self) -> f32 {
        self.f.unwrap_or(0.0)
    }
}

impl Manifolds {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn set(&mut self, manifolds: Vec<Manifold>) {
        self.0 = manifolds;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Manifold> {
        self.0.iter()
    }
}
