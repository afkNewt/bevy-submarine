use bevy::prelude::*;
use rand::distr::Bernoulli;
use resources::{ChunksPendingRebuild, Map};
use systems::{draw_on_map, regenerate_chunks, setup_map};

pub mod components;
pub mod resources;
pub mod systems;

pub mod chunk;

pub const SQUARE_SIZE: f32 = 10.;

pub const WATER_COLOR: Color = Color::hsl(230.0, 0.4, 0.3);
pub const WALL_COLOR: Color = Color::hsl(230.0, 0.1, 0.3);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(8, 4, Bernoulli::new(0.48).unwrap(), 4, 50, 500))
            .insert_resource(ChunksPendingRebuild::default())
            .add_systems(Startup, setup_map)
            .add_systems(Update, draw_on_map)
            .add_systems(Update, regenerate_chunks);
    }
}
