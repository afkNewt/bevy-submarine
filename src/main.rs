use bevy::prelude::*;
use terrain::{TerrainPlugin, WALL_COLOR};

mod terrain;

fn main() {
    App::new()
        .insert_resource(ClearColor(WALL_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
