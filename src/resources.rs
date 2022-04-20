use bevy::prelude::*;
use prima::Collision;

#[derive(Debug, Clone)]
pub struct BroadPhasePairs {
    pub pairs: Vec<(Entity, Entity)>,
}

pub struct Manifolds(Vec<Manifold>);

pub struct Manifold {
    pub a: Entity,
    pub b: Entity,
    pub collision: Collision<f32>,
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