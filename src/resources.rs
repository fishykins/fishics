use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, widgets::ResourceInspector};
use prima::{Collision, Vector};

#[derive(Inspectable, Default)]
pub struct Resources {
    config: ResourceInspector<FishicsConfig>,
}

#[derive(Debug, Clone)]
pub struct BroadPhasePairs {
    pub pairs: Vec<(Entity, Entity)>,
}

#[derive(Debug, Clone, Inspectable)]
pub struct FishicsConfig {
    pub scale: f32,
    max_speed: f32,
    #[inspectable(ignore)]
    max_speed_squared: f32,
}

#[derive(Debug, Clone)]
pub struct Manifolds(Vec<Manifold>);
#[derive(Debug, Clone)]
pub struct Manifold {
    pub a: Entity,
    pub b: Entity,
    pub n: Vector<f32>,
    pub p: f32,
    f: Option<f32>,
}

impl BroadPhasePairs {
    pub fn new() -> Self {
        BroadPhasePairs { pairs: Vec::new() }
    }
}

impl Manifold {
    pub fn new(a: Entity, b: Entity, collision: Collision<f32>) -> Self {
        let n = collision.normal.normalize();
        let p = collision.penetration;
        Self {
            a,
            b,
            n,
            p,
            f: None,
        }
    }

    pub fn with_initial_force(&self, f: f32) -> Self {
        let mut new = self.clone();
        new.f = Some(f);
        new
    }

    pub fn f(&self) -> f32 {
        self.f.unwrap_or(0.0)
    }
}

impl Manifolds {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn set(&mut self, manifolds: Vec<Manifold>) {
        self.0 = manifolds;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Manifold> {
        self.0.iter()
    }
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
