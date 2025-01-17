use bevy::prelude::*;

use crate::terrain::{components::MarchingSquares, WALL_COLOR};

use super::{
    chunk::{ChunkMap, CHUNK_SIZE},
    resources::Map,
    SQUARE_SIZE, WATER_COLOR,
};

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
) {
    let terrain = ChunkMap::new(map.points.to_owned(), SQUARE_SIZE);
    let mesh_handles = terrain.all_chunk_meshes(&mut meshes);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            (map.width - 2) as f32 * SQUARE_SIZE,
            (map.height - 2) as f32 * SQUARE_SIZE,
        ))),
        MeshMaterial2d(materials.add(WATER_COLOR)),
    ));

    const CHUNK_LENGTH: f32 = CHUNK_SIZE as f32 * SQUARE_SIZE;
    for x in 0..terrain.map.len() {
        for y in 0..terrain.map[x].len() {
            commands.spawn((
                Mesh2d(mesh_handles[x][y].clone()),
                MeshMaterial2d(materials.add(WALL_COLOR)),
                Transform::from_translation(Vec3::new(
                    x as f32 * CHUNK_LENGTH - (map.width as f32 / 16. * CHUNK_LENGTH) / 2.
                        + CHUNK_LENGTH / 2.,
                    y as f32 * CHUNK_LENGTH - (map.height as f32 / 16. * CHUNK_LENGTH) / 2.
                        + CHUNK_LENGTH / 2.,
                    1.,
                )),
                MarchingSquares,
            ));
        }
    }
}
