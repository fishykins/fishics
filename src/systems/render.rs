use crate::{AbstractShape, components::{ColliderRender, Collider}, resources::FishicsConfig};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn create_mesh_renders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cfg: Res<FishicsConfig>,
    colliders: Query<&Collider>,
    new_renderables: Query<(Entity, &ColliderRender), Without<Handle<Mesh>>>,
) {
    for (entity, col_renderer) in new_renderables.iter() {
        if let Some(collider) = colliders.get(entity).ok() {
            if let Some((mesh, scale)) = generate_mesh(collider.shape) {
                let transform = Transform::from_scale(scale * cfg.scale);

                let bundle = MaterialMesh2dBundle {
                    mesh: meshes.add(mesh).into(),
                    material: materials.add(col_renderer.colour.into()),
                    transform,
                    ..Default::default()
                };

                commands.entity(entity).insert_bundle(bundle);
            }
        }
    }
}

pub fn update_mesh_renders() {}

// ============================================================================
// ============================================================================

fn generate_mesh(shape: AbstractShape) -> Option<(Mesh, Vec3)> {
    match shape {
        AbstractShape::Circle { radius } => {
            let mesh = crate::build_circle(radius, 32);
            Some((mesh, Vec3::new(1.0, 1.0, 1.0)))
        }
        AbstractShape::Aabr { width, height } => {
            let mesh = Mesh::from(shape::Quad {
                size: Vec2::new(1.0, 1.0),
                flip: false,
            });
            let scale = Vec3::new(width, height, 1.0);
            Some((mesh, scale))
        }
        AbstractShape::Line { start: _, end: _ } => None,
    }
}
