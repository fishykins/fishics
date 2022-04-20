use bevy_inspector_egui::RegisterInspectable;

use crate::components::*;
use crate::systems::*;
use crate::BroadPhasePairs;
use crate::Manifolds;
use bevy::prelude::*;

pub struct FishicsPlugin {
    pub use_default_broad_phase: bool,
}

impl Default for FishicsPlugin {
    fn default() -> Self {
        Self {
            use_default_broad_phase: true,
        }
    }
}

impl Plugin for FishicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<RigidBody>()
            .register_inspectable::<Collider>()
            .register_inspectable::<Forces>()
            .register_inspectable::<Velocity>()
            .register_inspectable::<Mass>()
            .register_inspectable::<Inertia>()
            .register_inspectable::<PhysicsMaterial>();

        app.insert_resource(BroadPhasePairs::new())
            .insert_resource(Manifolds::new());

        app.add_system(integration.before(narrow_phase))
            .add_system(narrow_phase.before(impulse_resolution));

        if self.use_default_broad_phase {
            app.add_system(broad_phase.before(narrow_phase).after(integration));
        }

        app.add_system(impulse_resolution);
    }
}
