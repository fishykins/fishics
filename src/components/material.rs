use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct PhysicsMaterial {
    pub density: f32,
    pub restitution: f32,
    pub static_friction: f32,
    pub dynamic_friction: f32,
}

impl PhysicsMaterial {
    pub fn new(density: f32, restitution: f32, static_friction: f32, dynamic_friction: f32) -> Self {
        Self {
            density,
            restitution,
            static_friction,
            dynamic_friction,
        }
    }

    pub fn bouncy() -> Self {
        Self::new(0.3, 0.8, 0.9, 0.9)
    }

    pub fn hard() -> Self {
        Self::new(0.6, 0.1, 0.4, 0.5)
    }

    pub fn soft() -> Self {
        Self::new(0.1, 0.2, 0.2, 0.1)
    }

    pub fn r#static() -> Self {
        Self::new(0.0, 0.4, 1.0, 1.0)
    }

    pub fn metallic() -> Self {
        Self::new(1.2, 0.05, 0.01, 0.1)
    }

    pub fn wooden() -> Self {
        Self::new(0.3, 0.2, 0.2, 0.1)
    }
}

impl Default for PhysicsMaterial {
    fn default() -> Self {
        Self::wooden()
    }
}