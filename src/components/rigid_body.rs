use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::Point2;

#[derive(Debug, Clone, Copy, Default, Component, Inspectable)]
pub struct RigidBody {
    #[inspectable(ignore)]
    pub position: Point2,
    pub rotation: f32,
}
