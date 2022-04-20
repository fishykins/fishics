use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::{Aabr, Circle, Point2, Shape2};
use crate::{RigidBody, AbstractShape};

pub const DEFAULT_LAYER: u8 = 0b0000_0001;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct Collider {
    pub shape: AbstractShape,
    pub layer: u8,
}

impl Collider {
    pub fn square(size: f32) -> Self {
        Self {
            shape: AbstractShape::Aabr {
                half_extents: (size / 2.0, size / 2.0),
            },
            layer: DEFAULT_LAYER,
        }
    }

    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            shape: AbstractShape::Aabr {
                half_extents: (width / 2.0, height / 2.0),
            },
            layer: DEFAULT_LAYER,
        }
    }

    pub fn circle(radius: f32) -> Self {
        Self {
            shape: AbstractShape::Circle { radius },
            layer: DEFAULT_LAYER,
        }
    }

    pub fn with_layers(mut self, layers: u8) -> Self {
        self.layer = layers;
        self
    }

    pub fn global_aabr(&self, rb: &RigidBody) -> Aabr {
        match self.shape {
            AbstractShape::Circle { radius } => Circle::new(rb.position(), radius).bounding_box(),
            AbstractShape::Aabr { half_extents } => {
                let he = Point2::from(half_extents);
                Aabr::new(rb.position() - he, rb.position() + he)
            }
        }
    }

    pub fn as_shape(&self, rb: &RigidBody) -> Box<dyn Shape2<f32>> {
        match self.shape {
            AbstractShape::Circle { radius } => Box::new(Circle::new(rb.position(), radius)),
            AbstractShape::Aabr { half_extents } => {
                let he = Point2::from(half_extents);
                Box::new(Aabr::new(rb.position() - he, rb.position() + he))
            }
        }
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            shape: AbstractShape::Circle { radius: 0.5 },
            layer: DEFAULT_LAYER,
        }
    }
}
