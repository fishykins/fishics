use bevy::prelude::Bundle;

use crate::components::*;

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    pub rb: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub velocity: Velocity,
    pub forces: Forces,
    pub material: PhysicsMaterial,
    pub render: ColliderRender,
}