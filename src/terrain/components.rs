use bevy::prelude::*;

#[derive(Component)]
pub struct TerrainMesh {
    pub chunk_position: UVec2,
}

impl TerrainMesh {
    pub fn new(chunk_position: UVec2) -> Self {
        Self { chunk_position }
    }
}
