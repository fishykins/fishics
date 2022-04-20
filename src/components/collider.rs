use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::{Aabr, Circle, Shape2, Point2};

use crate::RigidBody;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct Collider {
    pub shape: ColliderShape,
    pub layer: u8,
}

#[derive(Debug, Clone, Copy, Inspectable)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Aabr { half_extents: (f32, f32) },
}

impl Collider {
    pub fn global_aabr(&self, rb: &RigidBody) -> Aabr {
        match self.shape {
            ColliderShape::Circle { radius } => {
                Circle::new(rb.position, radius).bounding_box()
            }
            ColliderShape::Aabr { half_extents } => {
                let he = Point2::from(half_extents);
                Aabr::new(rb.position - he, rb.position + he)
            }
        }
    }

    pub fn as_shape(&self, rb: &RigidBody) -> Box<dyn Shape2<f32>> {
        match self.shape {
            ColliderShape::Circle { radius } => {
                Box::new(Circle::new(rb.position, radius))
            },
            ColliderShape::Aabr { half_extents } => {
                let he = Point2::from(half_extents);
                Box::new(Aabr::new(rb.position - he, rb.position + he))
            }
        }
    }
}