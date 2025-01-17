use bevy::prelude::*;
use rand::distributions::Bernoulli;
use resources::Map;
use systems::setup_map;

pub mod components;
pub mod resources;
pub mod systems;

pub mod chunk;

const SQUARE_SIZE: f32 = 6.;

pub const WATER_COLOR: Color = Color::hsl(230.0, 0.4, 0.3);
pub const WALL_COLOR: Color = Color::hsl(230.0, 0.1, 0.3);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(12, 7, Bernoulli::new(0.48).unwrap(), 4, 50, 500))
            .add_systems(Startup, setup_map);
    }
}
