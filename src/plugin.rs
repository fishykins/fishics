use crate::components::*;
use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct FishicsPlugin {
    pub apply_transforms: bool,
    pub render_colliders: bool,
    pub use_default_broad_phase: bool,
    pub config: FishicsConfig,
}

impl Default for FishicsPlugin {
    fn default() -> Self {
        Self {
            apply_transforms: true,
            render_colliders: true,
            use_default_broad_phase: true,
            config: FishicsConfig::default(),
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
            .register_inspectable::<PhysicsMaterial>()
            .register_inspectable::<ColliderRender>();

        app.insert_resource(BroadPhasePairs::new())
            .insert_resource(Manifolds::new())
            .insert_resource(self.config.clone());

        app.add_asset::<PhysicsMaterial>();

        app.add_system(integration.before(narrow_phase))
            .add_system(narrow_phase.before(impulse_resolution));

        if self.use_default_broad_phase {
            app.add_system(broad_phase.before(narrow_phase).after(integration));
        }

        if self.config.max_speed() > 0.0 {
            app.add_system(speed_limmit.before(integration));
        }

        if self.apply_transforms {
            app.add_system(apply_transforms.after(integration));
        }

        if self.render_colliders {
            app.add_system_to_stage(CoreStage::PreUpdate, create_mesh_renders.after(integration));
        }

        app.add_system(impulse_resolution);
    }
}
