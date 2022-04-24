use bevy_inspector_egui::{Inspectable, widgets::ResourceInspector};

#[derive(Inspectable, Default)]
pub struct Resources {
    fishics: ResourceInspector<FishicsConfig>,
}


#[derive(Debug, Clone, Inspectable)]
pub struct FishicsConfig {
    pub scale: f32,
    #[inspectable(ignore)]
    max_speed: f32,
    #[inspectable(ignore)]
    max_speed_squared: f32,
}

impl Default for FishicsConfig {
    fn default() -> Self {
        Self {
            scale: 10.0,
            max_speed: 0.0,
            max_speed_squared: 0.0,
        }
    }
}

impl FishicsConfig {
    pub fn set_speed_limmit(&mut self, speed_limmit: f32) {
        self.max_speed = speed_limmit;
        self.max_speed_squared = speed_limmit * speed_limmit;
    }

    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }

    pub fn max_speed_squared(&self) -> f32 {
        self.max_speed_squared
    }
}
