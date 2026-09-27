use crate::draw::Position;

pub fn hash(coords: &Position, seed: &u64) -> u64 {
    let mut h = seed
        ^ (coords.0 as u64).wrapping_mul(0x9E3779B97F4A7C15)
        ^ (coords.1 as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51AFD7ED558CCD);
    h ^= h >> 33;
    h
}
