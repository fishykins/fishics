use bevy::prelude::{Bundle, Handle};
use crate::{components::*};

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    pub rb: RigidBody,
    pub collider: Collider,
    pub mass: Mass,
    pub velocity: Velocity,
    pub forces: Forces,
    pub properties: Handle<PhysicsMaterial>,
    pub render: ColliderRender,
}

#[derive(Bundle, Default)]
pub struct StaticRigidBodyBundle {
    pub rb: RigidBody,
    pub collider: Collider,
    pub properties: Handle<PhysicsMaterial>,
    pub render: ColliderRender,
}

