use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::Vector;

#[derive(Debug, Clone, Default, Component, Inspectable)]
pub struct Forces {
    x: f32,
    y: f32,
}

impl Forces {
    pub fn new(linear: Vector<f32>) -> Self {
        Self {
            x: linear.x,
            y: linear.y,
        }
    }

    pub fn resultant(&self) -> Vector<f32> {
        Vector::new(self.x, self.y)
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn collect(&mut self) -> Vector<f32> {
        let resultant = self.resultant();
        self.clear();
        resultant
    }
}
