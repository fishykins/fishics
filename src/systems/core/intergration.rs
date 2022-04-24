use bevy::prelude::*;
use prima::Rotation;

use crate::components::{RigidBody, Forces, Velocity, Mass};


/// Apply pending forces and rotations, as well as normalize any skewed values.
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
        velocity.add_linear(force.collect_impulse() * mass.inv() * dt);
        rb.translate(velocity.linear() * dt);

        // Normalize the rotation
        let r = Rotation::from_radians(rb.rotation);
        rb.rotation = r.as_rads();
    }
}
