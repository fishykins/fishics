use crate::components::*;
use crate::pipeline::*;
use crate::resources::*;
use crate::systems::{core::*, render::*, transforms::*};
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct FishicsPlugin {
    pub apply_transforms: bool,
    pub render_colliders: bool,
    pub config: FishicsConfig,
}

impl Default for FishicsPlugin {
    fn default() -> Self {
        Self {
            apply_transforms: true,
            render_colliders: true,
            config: FishicsConfig::default(),
        }
    }
}

impl Plugin for FishicsPlugin
{
    fn build(&self, app: &mut App) {
        app.register_inspectable::<RigidBody>()
            .register_inspectable::<Collider>()
            .register_inspectable::<Forces>()
            .register_inspectable::<Velocity>()
            .register_inspectable::<Mass>()
            .register_inspectable::<Inertia>()
            .register_inspectable::<PhysicsMaterial>()
            .register_inspectable::<ColliderRender>()
            .register_inspectable::<ClassicImpulseResolver>();

        app.insert_resource(BroadPhasePairs::new())
            .insert_resource(Manifolds::new())
            .insert_resource(self.config.clone())
            .insert_resource(ClassicImpulseResolver::default());

        app.add_asset::<PhysicsMaterial>();

        app.add_system(integration.before(narrow_phase))
            .add_system(narrow_phase.before(impulse_resolution::<ClassicImpulseResolver>))
            .add_system(broad_phase.before(narrow_phase).after(integration));

        if self.config.max_speed() > 0.0 {
            app.add_system(speed_limmit.before(integration));
        }

        if self.apply_transforms {
            app.add_system(apply_transforms.after(integration));
        }

        if self.render_colliders {
            app.add_system_to_stage(CoreStage::PreUpdate, create_mesh_renders.after(integration));
        }

        app.add_system(impulse_resolution::<ClassicImpulseResolver>);
    }
}
