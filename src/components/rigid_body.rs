use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::prelude::*;

#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct RigidBody {
    /// Position of the center of mass in world space.
    pub position: Vec2,
    /// Rotation in radians. Pi is NOT applied to this value, so it is in the range 0.0f32 -> 2.0f32.
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

    pub fn position(&self) -> Point<f32> {
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

    /// The 'true' value of rotation with pi applied. 
    pub fn applied_rotation(&self) -> f32 {
        self.rotation * std::f32::consts::PI
    }
}