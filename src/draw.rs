use rand::seq::IndexedRandom;
use crate::player::get_player_position;

pub const CHUNK_WIDTH: usize = 18;
pub const CHUNK_HEIGHT: usize = 10;
#[derive(Clone, Copy)]
pub enum Block {
    Air,
    Cloud,
    Grass,
    Dirt,
    Stone,
    Wood,
    TreeLeaf
}

pub type Rgb = (u8, u8, u8);

pub type Texture = [[ Rgb; 4]; 4];

pub type Chunk = [[Block; CHUNK_WIDTH]; CHUNK_HEIGHT];

pub type State = [Chunk; 3];

const SKY: Rgb = (135, 206, 235);
const GRASS: Rgb = (76, 175, 80);
const GRASS_DARK: Rgb = (46, 125, 50);
const DIRT: Rgb = (134, 96, 67);
const DIRT_DARK: Rgb = (100, 60, 30);
const STONE: Rgb = (128, 128, 128);
const STONE_DARK: Rgb = (90, 90, 90);
const WOOD: Rgb = (128, 100, 50);
const WOOD_DARK: Rgb = (90, 70, 30);
const LEAF: Rgb = (100, 150, 100);
const LEAF_DARK: Rgb = (70, 100, 70);
const CLOUD: Rgb = (200, 200, 200);
const CLOUD_DARK: Rgb = (150, 150, 150);
const LEAF_COLORS: [Rgb; 2] = [LEAF, LEAF_DARK];
const WOOD_COLORS: [Rgb; 2] = [WOOD, WOOD_DARK];
const STONE_COLORS: [Rgb; 2] = [STONE, STONE_DARK];
const DIRT_COLORS: [Rgb; 2] = [DIRT, DIRT_DARK];
const GRASS_COLORS: [Rgb; 4] = [GRASS, GRASS_DARK, DIRT_DARK, DIRT];
const CLOUD_COLORS: [Rgb; 2] = [CLOUD, CLOUD_DARK];
const SKY_COLORS: [Rgb; 1] = [SKY];


fn get_texture(block: Block) -> Texture {
    match block {
        Block::Dirt => generate_texture(Block::Dirt),
        Block::Grass => generate_texture(Block::Grass),
        Block::Stone => generate_texture(Block::Stone),
        Block::Wood => generate_texture(Block::Wood),
        Block::TreeLeaf => generate_texture(Block::TreeLeaf),
        Block::Cloud => generate_texture(Block::Cloud),
        Block::Air => generate_texture(Block::Air),
    }
}

fn get_color_pallet(block: Block) -> &'static [Rgb]  {
    match block {
        Block::Air => &SKY_COLORS,
        Block::Cloud => &CLOUD_COLORS,
        Block::Grass => &GRASS_COLORS,
        Block::Dirt => &DIRT_COLORS,
        Block::Stone => &STONE_COLORS,
        Block::Wood => &WOOD_COLORS,
        Block::TreeLeaf => &LEAF_COLORS,
    }
}

fn generate_texture(block: Block) -> Texture {
    let mut rng = rand::rng();
    let mut texture = Texture::default();
    let pallet: &'static [Rgb] = get_color_pallet(block);
    for row in 0..4 {
        for col in 0..4 {
            texture[row][col] = *pallet.choose(&mut rng).unwrap();
        }
    }
    texture
}


pub fn display_view(active_chunk: &Chunk) {
    for row in active_chunk {
        let mut textures = [Texture::default(); 20];
        for i in 0..CHUNK_WIDTH {
            textures[i] = get_texture(row[i]);
        }

        for line in 0..4 {
            for i in 0..CHUNK_WIDTH {
                for color in textures[i][line] {
                    print!("\x1b[38;2;{};{};{}m██\x1b[0m", color.0, color.1, color.2);
                }
            }
            println!();
        }
    }
}