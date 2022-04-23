use crate::{Manifold, Mass, RigidBody, Velocity};
use bevy::prelude::*;
use prima::{Point, Vector};

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
}

/// Result of a collision on an object.
#[derive(Debug, Clone, Copy)]
pub struct ImpulseResult {
    /// new linear velocity
    pub v: Vector<f32>,
    /// new angular velocity
    pub r: f32,
}

// ============================================================================
// ============================= RESOLUTION ===================================
// ============================================================================

/// Calculates the resulting velocities of two objects after a collision.
pub fn resolve_impulse(
    m: Manifold,
    a: ImpulseObject,
    b: ImpulseObject,
    _dt: f32,
) -> (ImpulseResult, ImpulseResult) {

    let r1: ImpulseResult = a.into();
    let r2: ImpulseResult = b.into();

    if m.f() <= 0. {
        return (r1, r2);
    }

    println!("boop");
    ImpulseResult::none()
}

// ============================================================================
// ============================================================================

pub fn generate_impulse_pair(
    manifold: &Manifold,
    v: &mut Query<&mut Velocity>,
    m: &Query<&Mass>,
    rb: &Query<&RigidBody>,
) -> (ImpulseObject, ImpulseObject) {
    let vel_1 = v.get(manifold.a).ok();
    let vel_2 = v.get(manifold.b).ok();
    let mass_1 = m.get(manifold.a).ok();
    let mass_2 = m.get(manifold.b).ok();

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

    let a = ImpulseObject {
        m: m1,
        i: i1,
        v: v1,
        r: a1,
        c: Point::new(com1.x, com1.y),
    };

    let b = ImpulseObject {
        m: m2,
        i: i2,
        v: v2,
        r: a2,
        c: Point::new(com2.x, com2.y),
    };

    (a, b)
}

impl ImpulseResult {
    pub fn new(v: Vector, r: f32) -> Self {
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
