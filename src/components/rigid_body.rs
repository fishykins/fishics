use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::{Point2, Vector2};

#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct RigidBody {
    pub position: Vec2,
    pub rotation: f32,
}

impl RigidBody {
    pub fn new(position: Point2<f32>) -> Self {
        Self {
            position: Vec2::new(position.x, position.y),
            rotation: 0.0,
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn position(&self) -> Point2 {
        Point2::new(self.position.x, self.position.y)
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position.x = position.x;
        self.position.y = position.y;
    }

    pub fn translate(&mut self, translation: Vector2<f32>) {
        self.position.x += translation.x;
        self.position.y += translation.y;
    }
}