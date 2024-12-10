use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

mod generate_vertices;
mod map_generator;

#[derive(Component)]
struct MarchingSquares;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(230., 0.4, 0.3)))
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

    let (positions, normals, uvs, indices) = generate_vertices::generate_vertices(1.);

    let mut new_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    new_mesh.insert_indices(Indices::U32(indices));

    let mesh_handle = meshes.add(new_mesh);

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(Color::hsl(230.0, 0.1, 0.3))),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        MarchingSquares,
    ));
}
