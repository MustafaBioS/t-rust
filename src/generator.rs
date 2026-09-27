use crate::draw::Block::Chest;
use crate::draw::{Block, CHUNK_HEIGHT, CHUNK_WIDTH, Chunk, Position, TextureMap};
use crate::player::Player;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::cmp::min;

pub struct CoordHolder {
    x: i32,
    y: i32,
}

impl Default for CoordHolder {
    fn default() -> Self {
        CoordHolder { x: 0, y: 0 }
    }
}
impl CoordHolder {
    pub fn add(&mut self, coord_to_add: &CoordHolder) {
        self.x += coord_to_add.x;
        self.y += coord_to_add.y;
    }

    pub fn to_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

pub struct WorldContent {
    pub camera: CoordHolder,
    pub seed: u64,
    pub is_dirty: bool,
    pub state: Chunk,
    pub player: Player,
    pub texture_map: TextureMap,
    pub rng: ThreadRng,
}

const BASE_FLOOR_Y: u32 = 6;

impl Default for WorldContent {
    fn default() -> Self {
        Self {
            camera: CoordHolder { x: 0, y: 0 },
            seed: 0,
            state: [[Block::Air; CHUNK_WIDTH]; CHUNK_HEIGHT],
            is_dirty: false,
            player: Player::default(),
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

    fn above_or_below(&mut self, value: u32, coords: &CoordHolder) -> i32 {
        match u32_hash(&coords.to_position(), &self.seed) % 2 == 0 {
            true => -1 * value as i32,
            false => 1 * value as i32,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    fn generate_terrain_difference(&mut self, coords: &CoordHolder) -> i32 {
        (u32_hash(&coords.to_position(), &self.seed) % 3) as i32 - 1
    }
    fn generate_initial_chunk(&mut self) {
        let mut generator_coords: CoordHolder = CoordHolder::default();
        generator_coords.set_y(BASE_FLOOR_Y as i32);
        for col in 0..CHUNK_WIDTH {
            generator_coords.set_x(col as i32);
            let mut ground_base =
                generator_coords.y + self.generate_terrain_difference(&generator_coords);
            ground_base = ground_base.clamp(3, CHUNK_HEIGHT as i32 - 2);
            if col == 0 {
                self.state[ground_base as usize - 1][0] = Chest
            }
            self.state[ground_base as usize][col] = Block::Grass;
            let dirt_depth = u32_hash(&(col as i32, 1), &self.seed) % 2 + 1;
            for i in 1..=dirt_depth {
                let dirt_spot = min(CHUNK_HEIGHT as i32 - 1, ground_base + i as i32);
                self.state[dirt_spot as usize][col] = Block::Dirt;
            }
            let stone_depth = dirt_depth + ground_base as u32 + 1;
            for i in stone_depth as usize..CHUNK_HEIGHT {
                self.state[i][col] = Block::Stone;
            }
            generator_coords.set_y(ground_base);
        }
    }

    pub fn initialize_state(&mut self) {
        self.generate_seed();
        self.generate_initial_chunk();
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

fn u32_hash(position: &Position, seed: &u64) -> u32 {
    hash(&position, seed) as u32
}
