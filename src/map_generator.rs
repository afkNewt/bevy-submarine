use rand::{distributions::Bernoulli, prelude::Distribution};

pub const MAP_WIDTH: usize = 1920;
pub const MAP_HEIGHT: usize = 1080;

pub struct MapGenerator {
    pub map: [[bool; MAP_HEIGHT]; MAP_WIDTH],
    pub distribution: Bernoulli,
}

impl MapGenerator {
    pub fn new(distribution: Bernoulli, smoothing: usize) -> Self {
        let mut map_gen = Self {
            map: [[false; MAP_HEIGHT]; MAP_WIDTH],
            distribution,
        };

        map_gen.random_fill();
        for _ in 0..smoothing {
            map_gen.smooth_map();
        }

        return map_gen;
    }

    fn smooth_map(&mut self) {
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                let neighbors = self.get_nieghbor_wall_count(x as i32, y as i32);

                if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
                    continue;
                }

                if neighbors > 4 {
                    self.map[x][y] = true;
                } else if neighbors < 4 {
                    self.map[x][y] = false;
                }
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
            if *x < 0 || *x >= MAP_WIDTH as i32 || *y < 0 || *y >= MAP_HEIGHT as i32 {
                false
            } else {
                self.map[*x as usize][*y as usize]
            }
        })
        .count() as i32;
    }

    fn random_fill(&mut self) {
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
                    self.map[x][y] = true;
                } else {
                    self.map[x][y] = self.distribution.sample(&mut rand::thread_rng());
                }
            }
        }
    }
}
