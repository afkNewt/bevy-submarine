use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use map_generator::MapGenerator;
use rand::distributions::Bernoulli;
use terrain::Terrain;

mod map_generator;
mod terrain;

#[derive(Component)]
struct MarchingSquares;

const WATER_COLOR: Color = Color::hsl(230.0, 0.1, 0.3);
const WALL_COLOR: Color = Color::hsl(230.0, 0.4, 0.3);

fn main() {
    App::new()
        .insert_resource(ClearColor(WATER_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    let square_size = 8.;
    let width = 80;
    let height = 64;

    let map = MapGenerator::new(width, height, Bernoulli::new(0.48).unwrap(), 4, 50, 500);
    let terrain = Terrain::new(map.map, square_size);
    let all_meshes = terrain.all_chunk_meshes();

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            width as f32 * square_size,
            height as f32 * square_size,
        ))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
    ));

    let chunk_width = 16. * 8.;
    let chunk_height = 16. * 8.;
    for x in 0..terrain.map.len() {
        for y in 0..terrain.map[x].len() {
            let (positions, normals, uvs, indices) = all_meshes[x][y].to_owned();

            let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            new_mesh.insert_indices(Indices::U32(indices));

            let mesh_handle = meshes.add(new_mesh);

            commands.spawn((
                Mesh2d(mesh_handle),
                MeshMaterial2d(materials.add(WATER_COLOR)),
                Transform::from_translation(Vec3::new(
                    x as f32 * chunk_width - (width as f32 / 16. * chunk_width) / 2.
                        + chunk_width / 2.,
                    y as f32 * chunk_height - (height as f32 / 16. * chunk_height) / 2.
                        + chunk_height / 2.,
                    1.,
                )),
                MarchingSquares,
            ));
        }
    }
}
