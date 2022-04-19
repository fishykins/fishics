use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Stores the inverted mass of an object, as this is the most commonly used value by the physics engine.
#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(1.0 / mass)
    }

    pub fn inv(self) -> f32 {
        self.0
    }

    pub fn raw(self) -> f32 {
        1.0 / self.0
    }
}