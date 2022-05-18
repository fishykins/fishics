use crate::{AbstractShape, components::RigidBody};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::prelude::*;

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
                width: size, height: size,
            },
            layer: DEFAULT_LAYER,
        }
    }

    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            shape: AbstractShape::Aabr {
                width, height,
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

    pub fn line(start: Point<f32>, end: Point<f32>) -> Self {
        Self {
            shape: AbstractShape::Line {
                start: Vec2::new(start.x, start.y),
                end: Vec2::new(end.x, end.y),
            },
            layer: DEFAULT_LAYER,
        }
    }

    pub fn with_layers(mut self, layers: u8) -> Self {
        self.layer = layers;
        self
    }

    pub fn global_aabr(&self, rb: &RigidBody) -> Aabr<f32> {
        match self.shape {
            AbstractShape::Circle { radius } => Circle::new(rb.position(), radius).bounding_rect(),
            AbstractShape::Aabr { width, height } => {
                Aabr::from_point(rb.position(), width, height)
            }
            AbstractShape::Line { start: a, end: b } => {
                Aabr::new(Point::new(a.x, a.y), Point::new(b.x, b.y))
            }
        }
    }

    pub fn as_shape(&self, rb: &RigidBody) -> Box<dyn Shape<f32>> {
        match self.shape {
            AbstractShape::Circle { radius } => Box::new(Circle::new(rb.position(), radius)),
            AbstractShape::Aabr { width, height } => {
                Box::new(Aabr::from_point(rb.position(), width, height))
            }
            AbstractShape::Line { start: _, end: _ } => todo!(),
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
