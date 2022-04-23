use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::Vector;

#[derive(Debug, Clone, Default, Component, Inspectable)]
pub struct Forces {
    /// the horizontal impulse applied to the entity
    x: f32,
    /// the vertical impulse applied to the entity
    y: f32,
}

impl Forces {
    pub fn add_impulse(&mut self, impule: Vector<f32>) {
        self.x += impule.x;
        self.y += impule.y;
    }
    pub fn collect_impulse(&mut self) -> Vector<f32> {
        let impulse = Vector::new(self.x, self.y);
        self.x = 0.0;
        self.y = 0.0;
        impulse
    }
}
