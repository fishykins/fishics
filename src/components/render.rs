use bevy::prelude::{Color, Component};
use bevy_inspector_egui::Inspectable;

pub const DEFAULT_COLOR: Color = Color::ALICE_BLUE;

#[derive(Debug, Clone, Copy, Component, Inspectable)]
pub struct ColliderRender {
    pub colour: Color,
}

impl Default for ColliderRender {
    fn default() -> Self {
        DEFAULT_COLOR.into()
    }
}

impl From<Color> for ColliderRender {
    fn from(colour: Color) -> Self {
        Self { colour }
    }
}