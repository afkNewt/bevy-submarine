#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Chunk {
    pub points: [[bool; 16]; 16],
}

impl Chunk {
    pub fn new(mut map: Vec<Vec<bool>>) -> Self {
        while map.len() < 16 {
            map.push(vec![false; 16]);
        }

        for x in 0..16 {
            while map[x].len() < 16 {
                map[x].push(false);
            }
        }

        let mut points = [[false; 16]; 16];

        for x in 0..16 {
            for y in 0..16 {
                points[x][y] = map[x][y];
            }
        }

        Self { points }
    }

    pub fn generate_vertices(
        &self,
        square_size: f32,
    ) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
        let get_point_int = |x: usize, y: usize| -> u8 {
            if x >= 16 || y >= 16 {
                return 1;
            }
            return self.points[x][y] as u8;
        };

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        // Iterate over 4 grid points at a time
        for row in 0..16 {
            for col in 0..16 {
                let value = (get_point_int(col, row) * 8
                    + get_point_int(col + 1, row) * 4
                    + get_point_int(col + 1, row + 1) * 2
                    + get_point_int(col, row + 1) * 1) as u8;

                let left_x = col as f32 * square_size - (16.0 * square_size) / 2.;
                let top_y = row as f32 * square_size - (16.0 * square_size) / 2.;

                let right_x = left_x + square_size;
                let bottom_y = top_y + square_size;

                let top = 0.5;
                let right = 0.5;
                let bottom = 0.5;
                let left = 0.5;

                match value {
                    0 => {}
                    1 => {
                        // Top left corner
                        positions.push([left_x, top_y + square_size * left, 0.0]);
                        positions.push([left_x, bottom_y, 0.0]);
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);

                        for _ in 0..3 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    2 => {
                        // Top right corner
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);
                        positions.push([right_x, top_y + square_size * right, 0.0]);
                        positions.push([right_x, bottom_y, 0.0]);

                        for _ in 0..3 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    3 => {
                        // Top rectangle
                        positions.push([left_x, top_y + square_size * left, 0.0]);
                        positions.push([right_x, top_y + square_size * right, 0.0]);
                        positions.push([right_x, bottom_y, 0.0]);
                        positions.push([left_x, bottom_y, 0.0]);

                        for _ in 0..4 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    4 => {
                        // Bottom right corner
                        positions.push([right_x, top_y, 0.0]);
                        positions.push([right_x, top_y + square_size * right, 0.0]);
                        positions.push([left_x + square_size * top, top_y, 0.0]);

                        for _ in 0..3 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    5 => {
                        // Top left AND bottom right corners
                        // Top left corner
                        positions.push([left_x, top_y + square_size * left, 0.0]);
                        positions.push([left_x, bottom_y, 0.0]);
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);
                        // Bottom right corner
                        positions.push([right_x, top_y, 0.0]);
                        positions.push([right_x, top_y + square_size * right, 0.0]);
                        positions.push([left_x + square_size * top, top_y, 0.0]);

                        for _ in 0..6 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Top left corner
                        indices.push((positions.len() - 6) as u32);
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        // Bottom right corner
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    6 => {
                        // Right rectangle
                        positions.push([left_x + square_size * top, top_y, 0.0]);
                        positions.push([right_x, top_y, 0.0]);
                        positions.push([right_x, bottom_y, 0.0]);
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);

                        for _ in 0..4 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    7 => {
                        // The opposite of the bottom left corner, made from 3 triangles
                        positions.push([left_x, bottom_y, 0.0]); // Top left
                        positions.push([right_x, bottom_y, 0.0]); // Top right
                        positions.push([right_x, top_y, 0.0]); // Bottom right
                        positions.push([left_x + square_size * top, top_y, 0.0]); // Bottom center
                        positions.push([left_x, top_y + square_size * left, 0.0]); // Center left

                        for _ in 0..5 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Triangle 1
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        // Triangle 2
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        // Triangle 3
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    8 => {
                        // Bottom left corner
                        positions.push([left_x, top_y, 0.0]);
                        positions.push([left_x + square_size * top, top_y, 0.0]);
                        positions.push([left_x, top_y + square_size * left, 0.0]);

                        for _ in 0..3 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    9 => {
                        // Left rectangle
                        positions.push([left_x, top_y, 0.0]);
                        positions.push([left_x + square_size * top, top_y, 0.0]);
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);
                        positions.push([left_x, bottom_y, 0.0]);

                        for _ in 0..4 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    10 => {
                        // Top right AND bottom left corners
                        // Top right corner
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]);
                        positions.push([right_x, top_y + square_size * right, 0.0]);
                        positions.push([right_x, bottom_y, 0.0]);
                        // Bottom left corner
                        positions.push([left_x, top_y, 0.0]);
                        positions.push([left_x + square_size * top, top_y, 0.0]);
                        positions.push([left_x, top_y + square_size * left, 0.0]);

                        for _ in 0..6 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Top right corner
                        indices.push((positions.len() - 6) as u32);
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        // Bottom left corner
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    11 => {
                        // The opposite of the bottom right corner, made from 3 triangles
                        positions.push([right_x, bottom_y, 0.0]); // Top right
                        positions.push([left_x, bottom_y, 0.0]); // Top left
                        positions.push([left_x, top_y, 0.0]); // Bottom left
                        positions.push([left_x + square_size * top, top_y, 0.0]); // Bottom center
                        positions.push([right_x, top_y + square_size * right, 0.0]); // Center right

                        for _ in 0..5 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Triangle 1
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        // Triangle 2
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        // Triangle 3
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    12 => {
                        // Bottom rectangle
                        positions.push([right_x, top_y + square_size * right, 0.0]); // right
                        positions.push([left_x, top_y + square_size * left, 0.0]); // left
                        positions.push([left_x, top_y, 0.0]);
                        positions.push([right_x, top_y, 0.0]);

                        for _ in 0..4 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    13 => {
                        // Opposite of the top left corner, made from 3 triangles
                        positions.push([right_x, top_y, 0.0]); // Bottom right
                        positions.push([left_x, top_y, 0.0]); // Bottom left
                        positions.push([left_x, bottom_y, 0.0]); // Top left
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]); // Top center
                        positions.push([right_x, top_y + square_size * right, 0.0]); // Center right

                        for _ in 0..5 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Triangle 1
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        // Triangle 2
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        // Triangle 3
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    14 => {
                        // The opposite of the top left corner, made from 3 triangles
                        positions.push([left_x, top_y, 0.0]); // bottom left
                        positions.push([right_x, top_y, 0.0]); // bottom right
                        positions.push([right_x, bottom_y, 0.0]); // top right
                        positions.push([left_x + square_size * bottom, bottom_y, 0.0]); // top center
                        positions.push([left_x, top_y + square_size * left, 0.0]); // Center left

                        for _ in 0..5 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Triangle 1
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        // Triangle 2
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        // Triangle 3
                        indices.push((positions.len() - 5) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    15 => {
                        //Square
                        positions.push([left_x, top_y, 0.0]);
                        positions.push([right_x, top_y, 0.0]);
                        positions.push([right_x, bottom_y, 0.0]);
                        positions.push([left_x, bottom_y, 0.0]);

                        for _ in 0..4 {
                            normals.push([0.0, 0.0, 1.0]);
                            uvs.push([0.0, 0.0]);
                        }

                        // Triangle1
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 3) as u32);
                        indices.push((positions.len() - 2) as u32);
                        // Triangle2
                        indices.push((positions.len() - 4) as u32);
                        indices.push((positions.len() - 2) as u32);
                        indices.push((positions.len() - 1) as u32);
                    }
                    _ => {}
                };
            }
        }

        return (positions, normals, uvs, indices);
    }
}

pub struct Terrain {
    pub map: Vec<Vec<Chunk>>,
    pub square_size: f32,
}

impl Terrain {
    pub fn new(base_map: Vec<Vec<bool>>, square_size: f32) -> Self {
        let width = base_map[0].len();
        let height = base_map.len();

        // Chunk only supports 16x16
        let target_chunk_width = 16;
        let target_chunk_height = 16;

        let chunk_x_count = width / target_chunk_width;
        let chunk_y_count = height / target_chunk_height;

        let base_map = base_map.into_iter().flatten().collect::<Vec<bool>>();

        let map = (0..chunk_y_count)
            .into_iter()
            .map(|y| {
                (0..chunk_x_count)
                    .into_iter()
                    .map(|x| {
                        let points = (0..target_chunk_height)
                            .into_iter()
                            .map(|i| {
                                let start = x * target_chunk_width
                                    + i * width
                                    + y * width * target_chunk_width;
                                base_map[start..(start + target_chunk_width)].to_vec()
                            })
                            .collect::<Vec<Vec<bool>>>();

                        return Chunk::new(points);
                    })
                    .collect::<Vec<Chunk>>()
            })
            .collect::<Vec<Vec<Chunk>>>();

        Self { map, square_size }
    }

    pub fn chunk_mesh(
        &self,
        x: usize,
        y: usize,
    ) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
        return self.map[x][y].generate_vertices(self.square_size);
    }

    pub fn all_chunk_meshes(
        &self,
    ) -> Vec<Vec<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>)>> {
        let mut meshes: Vec<Vec<(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>)>> =
            vec![Vec::default(); self.map.len()];

        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                meshes[x].push(self.chunk_mesh(x, y));
            }
        }

        return meshes;
    }
}
