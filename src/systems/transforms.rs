
use bevy::prelude::*;

use crate::{resources::FishicsConfig, components::{Velocity, RigidBody, Mass}};

pub fn speed_limmit(cfg: Res<FishicsConfig>, mut vel: Query<&mut Velocity>) {
    for mut vel in vel.iter_mut() {
        let v = vel.linear();
        let speed = v.magnitude_squared();
        if speed > cfg.max_speed_squared() {
            vel.set_linear(v.normalize() * cfg.max_speed());
        }
    }
}

/// Applies the [RigidBody] values to bevy's [Transform].
pub fn apply_transforms(
    cfg: Res<FishicsConfig>,
    mut bodies: Query<(&mut Transform, &RigidBody, Option<&Mass>)>,
) {
    for (mut transform, rigid_body, mass) in bodies.iter_mut() {
        let z = if let Some(mass) = mass {
            mass.inv()
        } else {
            1.0
        };
        let pos = Vec3::new(
            rigid_body.position.x * cfg.scale,
            rigid_body.position.y * cfg.scale,
            z,
        );
        transform.translation = pos;
        transform.rotation = Quat::from_rotation_z(rigid_body.applied_rotation());
    }
}
