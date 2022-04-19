use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct BroadPhasePairs {
    pub pairs: Vec<(Entity, Entity)>,
}