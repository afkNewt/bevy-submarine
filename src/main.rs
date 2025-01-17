use bevy::prelude::*;
use map_generator::MapGenerator;
use rand::distributions::Bernoulli;
use terrain::Terrain;

mod map_generator;
mod terrain;

#[derive(Component)]
struct MarchingSquares;

const WATER_COLOR: Color = Color::hsl(230.0, 0.4, 0.3);
const WALL_COLOR: Color = Color::hsl(230.0, 0.1, 0.3);

fn main() {
    App::new()
        .insert_resource(ClearColor(WALL_COLOR))
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
    let square_size = 6.;
    let width = 194;
    let height = 114;

    let map = MapGenerator::new(width, height, Bernoulli::new(0.48).unwrap(), 4, 50, 500);
    let terrain = Terrain::new(map.map, square_size);
    let mesh_handles = terrain.all_chunk_meshes(&mut meshes);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            (width - 2) as f32 * square_size,
            (height - 2) as f32 * square_size,
        ))),
        MeshMaterial2d(materials.add(WATER_COLOR)),
    ));

    let chunk_width = 16. * square_size;
    let chunk_height = 16. * square_size;
    for x in 0..terrain.map.len() {
        for y in 0..terrain.map[x].len() {
            commands.spawn((
                Mesh2d(mesh_handles[x][y].clone()),
                MeshMaterial2d(materials.add(WALL_COLOR)),
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
