use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct Inertia(f32);

impl Inertia {
    pub fn new(inertia: f32) -> Self {
        Self(inertia)
    }

    pub fn inv(&self) -> f32 {
        1.0 / self.0
    }
}