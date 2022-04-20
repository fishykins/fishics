use bevy_inspector_egui::RegisterInspectable;

use crate::components::*;
use crate::systems::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct FishicsPlugin {}

impl Plugin for FishicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<RigidBody>()
            .register_inspectable::<Collider>()
            .register_inspectable::<Forces>()
            .register_inspectable::<Velocity>()
            .register_inspectable::<Mass>()
            .register_inspectable::<Inertia>()
            .register_inspectable::<PhysicsMaterial>();

        app.add_system(integration)
            .add_system(broad_phase.after(integration))
            .add_system(narrow_phase.after(broad_phase))
            .add_system(impulse_resolution.after(narrow_phase));
    }
}
