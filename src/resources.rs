use bevy::prelude::*;
use prima::Collision;

#[derive(Debug, Clone)]
pub struct BroadPhasePairs {
    pub pairs: Vec<(Entity, Entity)>,
}

#[derive(Debug, Clone)]
pub struct Manifolds(Vec<Manifold>);

#[derive(Debug, Clone)]
pub struct FishicsConfig {
    pub scale: f32,
    max_speed: f32,
    max_speed_squared: f32,
}

#[derive(Debug, Clone)]
pub struct Manifold {
    pub a: Entity,
    pub b: Entity,
    pub collision: Collision<f32>,
}

impl BroadPhasePairs {
    pub fn new() -> Self {
        BroadPhasePairs { pairs: Vec::new() }
    }
}

impl Manifold {
    pub fn new(a: Entity, b: Entity, collision: Collision<f32>) -> Self {
        Self { a, b, collision }
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