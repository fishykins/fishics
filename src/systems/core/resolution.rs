use prima::Dot;

use crate::pipeline::{ImpulseObject, ImpulseResult, Manifold};

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
    let mut r1: ImpulseResult = a.into();
    let mut r2: ImpulseResult = b.into();

    if m.f() <= 0. {
        return (r1, r2);
    }

    let rv_n = (a.v - b.v).dot(&m.n);
    let e = 1.0;
    let j = (-(1.0 + e) * rv_n) / (a.i + b.i);
    r1.v = a.v + (m.n * a.i * j);
    r2.v = b.v - (m.n * b.i * j);

    println!("boop");
    (r1, r2)
}

// ============================================================================
// ============================================================================
