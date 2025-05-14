use bevy::prelude::*;
use terrain::{chunk::CHUNK_SIZE, TerrainPlugin, SQUARE_SIZE, WALL_COLOR};

mod terrain;

fn main() {
    App::new()
        .insert_resource(ClearColor(WALL_COLOR))
        .add_plugins((DefaultPlugins,))
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // in the future camera pos should follow plyar, so main won't know it
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(
            7. * CHUNK_SIZE as f32 * SQUARE_SIZE / 2. + SQUARE_SIZE,
            3. * CHUNK_SIZE as f32 * SQUARE_SIZE / 2. + SQUARE_SIZE,
            0.,
        ),
    ));
}
