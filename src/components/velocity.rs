use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::Vector2;

#[derive(Debug, Clone, Component, Inspectable)]
pub struct Velocity {
    #[inspectable(ignore)]
    pub linear: Vector2<f32>,
    pub angular: f32,
}
