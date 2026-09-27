use crate::draw::{Block, CHUNK_HEIGHT, CHUNK_WIDTH, Position, State, TextureMap};
use crate::player::Player;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::cmp::{max, min};

pub struct CoordHolder {
    x: i32,
    y: i32,
}

pub struct WorldContent {
    pub camera: CoordHolder,
    pub seed: u64,
    pub rendered_chunks: State,
    pub player_position: Player,
    pub texture_map: TextureMap,
    pub rng: ThreadRng,
}

const BASE_FLOOR_Y: u32 = 6;

impl Default for WorldContent {
    fn default() -> Self {
        Self {
            camera: CoordHolder { x: 0, y: 0 },
            seed: 0,
            rendered_chunks: [[[Block::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; 3],
            player_position: Player {
                x: 0,
                y: 0,
                is_grounded: true,
                is_paused: false,
            },
            texture_map: TextureMap::default(),
            rng: rand::rng(),
        }
    }
}

impl WorldContent {
    pub fn generate_seed(&mut self) {
        self.seed = self.rng.next_u64();
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    fn above_or_below(&mut self, value: u32) -> i32 {
        match self.rng.next_u32() % 2 == 0 {
            true => -1 * value as i32,
            false => 1 * value as i32,
        }
    }

    fn generate_terrain_difference(&mut self, last_ground_y: u32) -> i32 {
        if last_ground_y == 0 {
            let difference: u32 = self.rng.next_u32() % 4;
            return self.above_or_below(difference);
        }
        let delta_terrain: u32 = last_ground_y.abs_diff(BASE_FLOOR_Y);
        if delta_terrain == 0 {
            return 0;
        }
        let difference: u32 = self.rng.next_u32() % delta_terrain;
        self.above_or_below(difference)
    }
    pub fn generate_initial_chunk(&mut self) {
        println!("Generating initial chunk");
        let mut last_ground_y: u32 = 0;
        for col in 0..CHUNK_WIDTH {
            let mut ground_base =
                BASE_FLOOR_Y as i32 + self.generate_terrain_difference(last_ground_y);
            ground_base = max(ground_base, 0);
            ground_base = min(ground_base, CHUNK_HEIGHT as i32 - 1);
            self.rendered_chunks[1][ground_base as usize][col] = Block::Grass;
            last_ground_y = ground_base as u32;
        }
    }
}

pub fn hash(coords: &Position, seed: &u64) -> u64 {
    let mut h = seed
        ^ (coords.0 as u64).wrapping_mul(0x9E3779B97F4A7C15)
        ^ (coords.1 as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51AFD7ED558CCD);
    h ^= h >> 33;
    h
}
