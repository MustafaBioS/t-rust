use crate::draw::{Block, CHUNK_HEIGHT, CHUNK_WIDTH, Position, State, TextureMap};
use crate::player::Player;
use rand::Rng;
use rand::rngs::ThreadRng;

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
