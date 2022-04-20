use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Stores the mass associated with an entity.
#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }

    pub fn inv(self) -> f32 {
        if self.0 == 0.0 {
            0.0
        } else {
            1.0 / self.0
        }
    }

    pub fn raw(self) -> f32 {
        self.0
    }
}

impl Default for Mass {
    fn default() -> Self {
        Self::new(100.0)
    }
}