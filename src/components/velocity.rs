use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::prelude::*;

#[derive(Debug, Clone, Component, Inspectable)]
pub struct Velocity {
    linear: Vec2,
    angular: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vec2::new(0.0, 0.0),
            angular: 0.0,
        }
    }
}

impl Velocity {
    pub fn new(linear: Vec2, angular: f32) -> Self {
        Self { linear, angular }
    }

    pub fn linear(&self) -> Vector<f32> {
        Vector::new(self.linear.x, self.linear.y)
    }

    pub fn magnitide_squared(&self) -> f32 {
        self.linear.x * self.linear.x + self.linear.y * self.linear.y
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitide_squared().sqrt()
    }

    pub fn set_linear(&mut self, linear: Vector<f32>) {
        self.linear.x = linear.x;
        self.linear.y = linear.y;
    }

    pub fn add_linear(&mut self, linear: Vector<f32>) {
        self.linear.x += linear.x;
        self.linear.y += linear.y;
    }

    pub fn sub_linear(&mut self, linear: Vector<f32>) {
        self.linear.x -= linear.x;
        self.linear.y -= linear.y;
    }

    pub fn angular(&self) -> f32 {
        self.angular
    }

    pub fn set_angular(&mut self, angular: f32) {
        self.angular = angular;
    }
}