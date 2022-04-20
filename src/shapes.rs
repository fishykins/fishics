use bevy_inspector_egui::Inspectable;
use prima::{Circle, Aabr, Point2, Interact};

/// A wrapper for all valid shapes.
#[derive(Debug, Clone, Copy)]
pub enum ShapeWrapper {
    Circle(Circle),
    Aabr(Aabr),
}

/// Just stores the shapes extremities without positional data.
#[derive(Debug, Clone, Copy, Inspectable)]
pub enum AbstractShape {
    Circle { radius: f32 },
    Aabr { half_extents: (f32, f32) },
}

impl ShapeWrapper {
    pub fn circle(center: Point2, radius: f32) -> Self {
        Self::Circle(Circle::new(center, radius))
    }

    pub fn aabr(center: Point2, half_extents: (f32, f32)) -> Self {
        Self::Aabr(Aabr::new(center - Point2::new(half_extents.0, half_extents.1), center + Point2::new(half_extents.0, half_extents.1)))
    }
}

impl AbstractShape {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    pub fn aabr(half_extents: (f32, f32)) -> Self {
        Self::Aabr { half_extents }
    }

    pub fn wrap(self, position: Point2) -> ShapeWrapper {
        match self {
            Self::Circle { radius } => ShapeWrapper::circle(position, radius),
            Self::Aabr { half_extents } => ShapeWrapper::aabr(position, half_extents),
        }
    }
}

impl Into<AbstractShape> for ShapeWrapper {
    fn into(self) -> AbstractShape {
        match self {
            Self::Circle(circle) => AbstractShape::circle(circle.radius),
            Self::Aabr(aabr) => AbstractShape::aabr(aabr.half_extents())
        }
    }
}

impl Interact<f32> for ShapeWrapper {
    fn collision(&self, other: &Self) -> Option<prima::Collision<f32>> {
        match self {
            ShapeWrapper::Circle(self_circle) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => self_circle.collision(other_circle),
                    ShapeWrapper::Aabr(other_aabr) => self_circle.collision(other_aabr),
                }
            },
            ShapeWrapper::Aabr(self_aabr) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.collision(self_aabr),
                    ShapeWrapper::Aabr(other_aabr) => self_aabr.collision(other_aabr),
                }
            },
        }
    }

    fn nearest_extent(&self, other: &Self) -> Point2<f32> {
        match self {
            ShapeWrapper::Circle(self_circle) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => self_circle.nearest_extent(other_circle),
                    ShapeWrapper::Aabr(other_aabr) => self_circle.nearest_extent(other_aabr),
                }
            },
            ShapeWrapper::Aabr(self_aabr) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.nearest_extent(self_aabr),
                    ShapeWrapper::Aabr(other_aabr) => self_aabr.nearest_extent(other_aabr),
                }
            },
        }
    }
}