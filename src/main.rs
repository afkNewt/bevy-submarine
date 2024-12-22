use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use rand::distributions::Bernoulli;

mod generate_vertices;
mod map_generator;

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

    let (positions, normals, uvs, indices) = generate_vertices::generate_vertices(
        8.,
        192,
        108,
        Bernoulli::new(0.48).unwrap(),
        4,
        50,
        500,
    );

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(192. * 8., 108. * 8.))),
        MeshMaterial2d(materials.add(WALL_COLOR)),
    ));

    let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    new_mesh.insert_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(new_mesh);

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(WATER_COLOR)),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        MarchingSquares,
    ));
}
