use bevy::math::Vec2;
use bevy_inspector_egui::Inspectable;
use prima::prelude::*;

/// A wrapper for all valid shapes.
#[derive(Debug, Clone)]
pub enum ShapeWrapper {
    Circle(Circle<f32>),
    Aabr(Aabr<f32>),
    Line(Line<f32>),
}

/// Just stores the shapes extremities without positional data.
#[derive(Debug, Clone, Copy, Inspectable)]
pub enum AbstractShape {
    Circle { radius: f32 },
    Aabr { half_extents: (f32, f32) },
    Line { start: Vec2, end: Vec2 },
}

impl ShapeWrapper {
    pub fn circle(center: Point<f32>, radius: f32) -> Self {
        Self::Circle(Circle::<f32>::new(center, radius))
    }

    pub fn aabr(center: Point<f32>, half_extents: (f32, f32)) -> Self {
        Self::Aabr(Aabr::from_point(center, half_extents.0 * 2.0, half_extents.1 * 2.0) )
    }

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self::Line(Line::new(Point::new(start.x, start.y), Point::new(end.x, end.y)))
    }
}

impl AbstractShape {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    pub fn aabr(half_extents: (f32, f32)) -> Self {
        Self::Aabr { half_extents }
    }

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self::Line { start, end }
    }

    pub fn wrap(self, position: Point<f32>) -> ShapeWrapper {
        match self {
            AbstractShape::Circle { radius } => ShapeWrapper::circle(position, radius),
            AbstractShape::Aabr { half_extents } => ShapeWrapper::aabr(position, half_extents),
            AbstractShape::Line { start, end } => ShapeWrapper::line(Vec2::new(position.x, position.y) + start, Vec2::new(position.x, position.y)  + end),
        }
    }
}

impl Into<AbstractShape> for ShapeWrapper {
    fn into(self) -> AbstractShape {
        match self {
            ShapeWrapper::Circle(circle) => AbstractShape::circle(circle.radius),
            ShapeWrapper::Aabr(aabr) => AbstractShape::aabr(aabr.extent().half().into()),
            ShapeWrapper::Line(line) => AbstractShape::line(Vec2::new(line.start.x, line.start.y), Vec2::new(line.end.x, line.end.y)),
        }
    }
}

impl Collide<f32> for ShapeWrapper {
    fn collision(&self, other: &Self) -> Option<Collision<f32>> {
        match self {
            ShapeWrapper::Circle(self_circle) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => self_circle.collision(other_circle),
                    ShapeWrapper::Aabr(other_aabr) => self_circle.collision(other_aabr),
                    ShapeWrapper::Line(other_line) => self_circle.collision(other_line),
                }
            },
            ShapeWrapper::Aabr(self_aabr) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.collision(self_aabr),
                    ShapeWrapper::Aabr(other_aabr) => self_aabr.collision(other_aabr),
                    ShapeWrapper::Line(other_line) => self_aabr.collision(other_line),
                }
            },
            ShapeWrapper::Line(self_line) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.collision(self_line),
                    ShapeWrapper::Aabr(other_aabr) => other_aabr.collision(self_line),
                    ShapeWrapper::Line(_) => None,
                }
            },
        }
    }

    fn enveloping(&self, other: &Self) -> bool {
        match self {
            ShapeWrapper::Circle(self_circle) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => self_circle.enveloping(other_circle),
                    ShapeWrapper::Aabr(other_aabr) => self_circle.enveloping(other_aabr),
                    ShapeWrapper::Line(other_line) => self_circle.enveloping(other_line),
                }
            },
            ShapeWrapper::Aabr(self_aabr) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.enveloping(self_aabr),
                    ShapeWrapper::Aabr(other_aabr) => self_aabr.enveloping(other_aabr),
                    ShapeWrapper::Line(other_line) => self_aabr.enveloping(other_line),
                }
            },
            ShapeWrapper::Line(self_line) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.enveloping(self_line),
                    ShapeWrapper::Aabr(other_aabr) => other_aabr.enveloping(self_line),
                    ShapeWrapper::Line(_) => false,
                }
            },
        }
    }

    fn enveloped_by(&self, other: &Self) -> bool {
        match self {
            ShapeWrapper::Circle(self_circle) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => self_circle.enveloped_by(other_circle),
                    ShapeWrapper::Aabr(other_aabr) => self_circle.enveloped_by(other_aabr),
                    ShapeWrapper::Line(other_line) => self_circle.enveloped_by(other_line),
                }
            },
            ShapeWrapper::Aabr(self_aabr) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.enveloped_by(self_aabr),
                    ShapeWrapper::Aabr(other_aabr) => self_aabr.enveloped_by(other_aabr),
                    ShapeWrapper::Line(other_line) => self_aabr.enveloped_by(other_line),
                }
            },
            ShapeWrapper::Line(self_line) => {
                match other {
                    ShapeWrapper::Circle(other_circle) => other_circle.enveloped_by(self_line),
                    ShapeWrapper::Aabr(other_aabr) => other_aabr.enveloped_by(self_line),
                    ShapeWrapper::Line(_) => false,
                }
            },
        }
    }
}