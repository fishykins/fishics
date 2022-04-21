use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::{Point, Vector};

#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct RigidBody {
    pub position: Vec2,
    pub rotation: f32,
}

impl RigidBody {
    pub fn new(position: Point<f32>) -> Self {
        Self {
            position: Vec2::new(position.x, position.y),
            rotation: 0.0,
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn position(&self) -> Point {
        Point::new(self.position.x, self.position.y)
    }

    pub fn set_position(&mut self, position: Point<f32>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }

    pub fn translate(&mut self, translation: Vector<f32>) {
        self.position.x += translation.x;
        self.position.y += translation.y;
    }
}