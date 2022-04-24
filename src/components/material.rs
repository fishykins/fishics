use bevy::reflect::TypeUuid;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, TypeUuid, Inspectable)]
#[uuid = "468845e7-5b30-4816-b3fd-22f1b4b73adc"]
pub struct PhysicsMaterial {
    /// Coefficient of restitution, or bounciness.
    /// 0.0 is a mushy material, 1.0 is a very bouncy material.
    #[inspectable(min = 0.0, max = 1.0)]
    pub restitution: f32,
}

impl PhysicsMaterial {
    pub fn new(restitution: f32) -> Self {
        Self { restitution }
    }

    pub fn bouncy() -> Self {
        Self::new(0.8)
    }

    pub fn hard() -> Self {
        Self::new(0.1)
    }
}

impl Default for PhysicsMaterial {
    fn default() -> Self {
        Self { restitution: 0.5 }
    }
}
