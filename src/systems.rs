use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use prima::{Interact, Intersect};

use crate::{
    AbstractShape, BroadPhasePairs, Collider, ColliderRender, FishicsConfig, Forces, Manifold,
    Manifolds, Mass, RigidBody, Velocity,
};

/// Steps the bodies.
pub fn integration(
    time: Res<Time>,
    mut bodies: Query<(&mut RigidBody, &mut Forces, &mut Velocity, &Mass)>,
) {
    let dt = time.delta_seconds();
    for (mut rb, mut force, mut velocity, mass) in bodies.iter_mut() {
        if mass.raw() == 0.0 {
            continue;
        }
        // Symplectic Euler integration. The order of the next two lines is important!
        velocity.add_linear(force.collect_impulse() * mass.inv() * dt);
        rb.translate(velocity.linear() * dt);
    }
}

/// Find potential collisions.
pub fn broad_phase(
    mut bf_pairs: ResMut<BroadPhasePairs>,
    bodies: Query<(Entity, &RigidBody, &Collider)>,
) {
    let mut pairs = Vec::new();

    // TODO: Quad trees!!!

    let mut combos = bodies.iter_combinations();
    while let Some([(entity_a, rb_a, c_a), (entity_b, rb_b, c_b)]) = combos.next() {
        if c_a.layer & c_b.layer == 0 {
            continue;
        }

        let b_a = c_a.global_aabr(rb_a);
        let b_b = c_b.global_aabr(rb_b);
        if b_a.intersecting(&b_b) {
            pairs.push((entity_a, entity_b));
        }
    }
    bf_pairs.pairs = pairs;
}

/// Find actual collisions.
pub fn narrow_phase(
    mut manifolds: ResMut<Manifolds>,
    pairs: Res<BroadPhasePairs>,
    bodies: Query<&RigidBody>,
    colliders: Query<&Collider>,
) {
    let mut new_manifolds = Vec::new();

    for (a, b) in pairs.pairs.iter() {
        let a_rb = bodies.get(*a).unwrap();
        let b_rb = bodies.get(*b).unwrap();
        let a_col = colliders.get(*a).unwrap();
        let b_col = colliders.get(*b).unwrap();

        let a_shape = a_col.shape.wrap(a_rb.position());
        let b_shape = b_col.shape.wrap(b_rb.position());

        if let Some(collision) = a_shape.collision(&b_shape) {
            //println!("{:?}", collision);
            new_manifolds.push(Manifold::new(*a, *b, collision));
        }
    }
    manifolds.set(new_manifolds);
}

pub fn impulse_resolution(
    time: Res<Time>,
    manifolds: Res<Manifolds>,
    rbq: Query<&RigidBody>,
    mq: Query<&Mass>,
    mut vq: Query<&mut Velocity>,
) {
    let dt = time.delta_seconds();

    for manifold in manifolds.iter() {
        // Collect impulse data.
        let (a, b) = crate::generate_impulse_pair(manifold, &mut vq, &mq, &rbq);

        // Calculate the initial force of the collision.
        let initial_force = a.m * a.v.magnitude() + b.m * b.v.magnitude();

        // Send impulse data to the collision resolution function.
        let (a, b) = crate::resolve_impulse(manifold.with_initial_force(initial_force), a, b, dt);

        // Apply impulses!
        let mut va = vq.get_mut(manifold.a).unwrap();
        va.set_linear(a.v);
        va.set_angular(a.r);
        let mut vb = vq.get_mut(manifold.b).unwrap();
        vb.set_linear(b.v);
        vb.set_angular(b.r);

        //std::process::exit(6002);
    }
}

pub fn speed_limmit(cfg: Res<FishicsConfig>, mut vel: Query<&mut Velocity>) {
    for mut vel in vel.iter_mut() {
        let v = vel.linear();
        let speed = v.magnitude_squared();
        if speed > cfg.max_speed_squared() {
            vel.set_linear(v.normalize() * cfg.max_speed());
        }
    }
}

pub fn apply_transforms(
    cfg: Res<FishicsConfig>,
    mut bodies: Query<(&mut Transform, &RigidBody, Option<&Mass>)>,
) {
    for (mut transform, rigid_body, mass) in bodies.iter_mut() {
        let z = if let Some(mass) = mass {
            mass.inv()
        } else {
            1.0
        };
        let pos = Vec3::new(
            rigid_body.position.x * cfg.scale,
            rigid_body.position.y * cfg.scale,
            z,
        );
        transform.translation = pos;
    }
}

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
        AbstractShape::Aabr { half_extents } => {
            let mesh = Mesh::from(shape::Quad {
                size: Vec2::new(2.0, 2.0),
                flip: false,
            });
            let scale = Vec3::new(half_extents.0, half_extents.1, 1.0);
            Some((mesh, scale))
        }
        AbstractShape::Line { start: _, end: _ } => None,
    }
}
