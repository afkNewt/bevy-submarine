use bevy::prelude::Resource;
use rand::{distributions::Bernoulli, prelude::Distribution};

use super::chunk::CHUNK_SIZE;

#[derive(Resource, Clone)]
pub struct Map {
    pub points: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(
        chunk_x: usize,
        chunk_y: usize,
        distribution: Bernoulli,
        smoothing: usize,
        min_wall_region_size: usize,
        min_air_region_size: usize,
    ) -> Self {
        let width = chunk_x * CHUNK_SIZE + 2;
        let height = chunk_y * CHUNK_SIZE + 2;

        let mut map_gen = Self {
            points: vec![vec![false; height]; width],
            width,
            height,
        };

        map_gen.clean_map(min_wall_region_size, min_air_region_size);

        map_gen.random_fill(distribution);
        for _ in 0..smoothing {
            map_gen.smooth_map();
        }

        map_gen.clean_map(min_wall_region_size, min_air_region_size);

        return map_gen;
    }

    fn smooth_map(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.get_nieghbor_wall_count(x as i32, y as i32);

                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    continue;
                }

                if neighbors > 4 {
                    self.points[x][y] = true;
                } else if neighbors < 4 {
                    self.points[x][y] = false;
                }
            }
        }
    }

    fn clean_map(&mut self, min_wall_region_size: usize, min_air_region_size: usize) {
        let regions = self.get_regions(true);

        for region in regions {
            if region.len() >= min_wall_region_size {
                continue;
            }

            for (x, y) in region {
                self.points[x][y] = false;
            }
        }

        let regions = self.get_regions(false);

        for region in regions {
            if region.len() >= min_air_region_size {
                continue;
            }

            for (x, y) in region {
                self.points[x][y] = true;
            }
        }
    }

    fn get_nieghbor_wall_count(&self, x: i32, y: i32) -> i32 {
        return [
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
        ]
        .iter()
        .filter(|(x, y)| {
            if !self.is_in_map(*x as usize, *y as usize) {
                true
            } else {
                self.points[*x as usize][*y as usize]
            }
        })
        .count() as i32;
    }

    fn get_region_tiles(&self, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let mut contiguous_tiles = Vec::new();
        let mut queued_tiles = Vec::new();
        let mut viewed_tiles = vec![vec![false; self.height]; self.width];
        let target_tile_type = self.points[start_x][start_y];

        queued_tiles.push((start_x, start_y));
        viewed_tiles[start_x][start_y] = true;

        while queued_tiles.len() > 0 {
            let (tile_x, tile_y) = queued_tiles.pop().unwrap();
            contiguous_tiles.push((tile_x, tile_y));

            for offset in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let target_x = tile_x.checked_add_signed(offset.0).unwrap_or_default();
                let target_y = tile_y.checked_add_signed(offset.1).unwrap_or_default();

                if !self.is_in_map(target_x, target_y) {
                    continue;
                }

                if self.points[target_x][target_y] != target_tile_type {
                    continue;
                }

                if !self.is_in_map(target_x, target_y) {
                    continue;
                }

                if viewed_tiles[target_x][target_y] {
                    continue;
                }

                viewed_tiles[target_x][target_y] = true;
                queued_tiles.push((target_x, target_y));
            }
        }

        return contiguous_tiles;
    }

    fn get_regions(&self, tile_type: bool) -> Vec<Vec<(usize, usize)>> {
        let mut regions = Vec::new();
        let mut viewed_tiles = vec![vec![false; self.height]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                if self.points[x][y] != tile_type {
                    continue;
                }

                if viewed_tiles[x][y] {
                    continue;
                }

                let new_region = self.get_region_tiles(x, y);

                for (x, y) in &new_region {
                    viewed_tiles[*x][*y] = true;
                }

                regions.push(new_region);
            }
        }

        return regions;
    }

    fn is_in_map(&self, x: usize, y: usize) -> bool {
        return !(x >= self.width || y >= self.height);
    }

    fn random_fill(&mut self, distribution: Bernoulli) {
        for x in 0..self.width {
            for y in 0..self.height {
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    self.points[x][y] = true;
                } else {
                    self.points[x][y] = distribution.sample(&mut rand::thread_rng());
                }
            }
        }
    }
}
