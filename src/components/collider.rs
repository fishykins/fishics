use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::{Aabr, Circle, Shape2, Point2};

use crate::RigidBody;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct Collider {
    pub shape: ColliderShape,
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum ColliderShape {
    None,
    Circle { radius: f32 },
    Aabb { half_extents: (f32, f32) },
}

impl Collider {
    pub fn global_aabr(&self, rb: &RigidBody) -> Aabr {
        match self.shape {
            ColliderShape::None => Aabr::new(rb.position, rb.position),
            ColliderShape::Circle { radius } => {
                Circle::new(rb.position, radius).bounding_box()
            }
            ColliderShape::Aabb { half_extents } => {
                let he = Point2::from(half_extents);
                Aabr::new(rb.position - he, rb.position + he)
            }
        }
    }
}