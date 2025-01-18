use bevy::{prelude::*, reflect::List, render::render_resource::encase::rts_array::Length};

use crate::terrain::{self, components::TerrainMesh, WALL_COLOR};

use super::{
    chunk::{ChunkMap, CHUNK_SIZE},
    resources::{ChunksPendingRebuild, Map},
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

    const CHUNK_LENGTH: f32 = CHUNK_SIZE as f32 * SQUARE_SIZE;
    let chunk_map_width = terrain.map.len();
    let chunk_map_height = terrain.map[0].len();

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            (map.width - 2) as f32 * SQUARE_SIZE,
            (map.height - 2) as f32 * SQUARE_SIZE,
        ))),
        MeshMaterial2d(materials.add(WATER_COLOR)),
        Transform::from_xyz(
            (map.width - 16) as f32 * SQUARE_SIZE / 2.,
            (map.height - 16) as f32 * SQUARE_SIZE / 2.,
            0.,
        ),
    ));

    for x in 0..chunk_map_width {
        for y in 0..chunk_map_height {
            commands.spawn((
                Mesh2d(mesh_handles[x][y].clone()),
                MeshMaterial2d(materials.add(WALL_COLOR)),
                Transform::from_translation(Vec3::new(
                    x as f32 * CHUNK_LENGTH,
                    y as f32 * CHUNK_LENGTH,
                    1.,
                )),
                TerrainMesh::new(UVec2::new(x as u32, y as u32)),
            ));
        }
    }
}

pub fn draw_on_map(
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_window: Query<&Window>,
    mut chunks_pending_rebuild: ResMut<ChunksPendingRebuild>,
    mut map: ResMut<Map>,
) {
    let (camera, camera_pos) = q_camera.single();
    let window = q_window.single();

    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_pos, cursor_pos).ok())
    else {
        return;
    };

    let Some(map_index) = map.world_space_to_index(cursor_pos) else {
        return;
    };

    let chunk_index = UVec2::new(map_index.0 as u32 / 16, map_index.1 as u32 / 16);
    chunks_pending_rebuild.chunks.push(chunk_index);
    // if we are on the edge of a chunk, then the neighbor must be updated, could be changed to only update
    // the neighbor that needs it
    chunks_pending_rebuild
        .chunks
        .push(UVec2::new(chunk_index.x.saturating_sub(1), chunk_index.y));
    chunks_pending_rebuild
        .chunks
        .push(UVec2::new(chunk_index.x + 1, chunk_index.y));
    chunks_pending_rebuild
        .chunks
        .push(UVec2::new(chunk_index.x, chunk_index.y.saturating_sub(1)));
    chunks_pending_rebuild
        .chunks
        .push(UVec2::new(chunk_index.x, chunk_index.y + 1));

    map.points[map_index.0][map_index.1] = false;
}

pub fn regenerate_chunks(
    mut commands: Commands,
    q_chunks: Query<(Entity, &TerrainMesh)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut chunks_pending_rebuild: ResMut<ChunksPendingRebuild>,
    map: Res<Map>,
) {
    if chunks_pending_rebuild.chunks.is_empty() {
        return;
    }

    let terrain = ChunkMap::new(map.points.to_owned(), SQUARE_SIZE);

    let mesh_handles = terrain.all_chunk_meshes(&mut meshes);
    const CHUNK_LENGTH: f32 = CHUNK_SIZE as f32 * SQUARE_SIZE;

    for (entity, terrain_mesh) in q_chunks.iter() {
        let x = terrain_mesh.chunk_position.x as usize;
        let y = terrain_mesh.chunk_position.y as usize;

        if !chunks_pending_rebuild
            .chunks
            .contains(&terrain_mesh.chunk_position)
        {
            continue;
        }

        commands.entity(entity).despawn();

        commands.spawn((
            Mesh2d(mesh_handles[x][y].clone()),
            MeshMaterial2d(materials.add(WALL_COLOR)),
            Transform::from_translation(Vec3::new(
                x as f32 * CHUNK_LENGTH,
                y as f32 * CHUNK_LENGTH,
                1.,
            )),
            TerrainMesh::new(UVec2::new(x as u32, y as u32)),
        ));
    }

    chunks_pending_rebuild.chunks.clear();
}
