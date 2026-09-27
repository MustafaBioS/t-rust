use crate::generator::hash;
use crate::player::Player;

pub const CHUNK_WIDTH: usize = 18;
pub const CHUNK_HEIGHT: usize = 10;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Block {
    Air,
    Cloud,
    Grass,
    Dirt,
    Stone,
    Wood,
    TreeLeaf,
}

pub type Rgb = (u8, u8, u8);

pub type Texture = [[Rgb; 4]; 4];

pub type Chunk = [[Block; CHUNK_WIDTH]; CHUNK_HEIGHT];

pub type Position = (i32, i32);

pub type TextureMap = std::collections::HashMap<Position, Texture>;

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

fn get_texture(
    block: Block,
    texture_map: &mut TextureMap,
    coords: &Position,
    seed: &u64,
) -> Texture {
    match texture_map.get(coords) {
        Some(texture) => return *texture,
        None => (),
    }

    let mut texture = Texture::default();
    let pallet: &'static [Rgb] = get_color_pallet(block);
    for row in 0..4 {
        for col in 0..4 {
            if row == 0 && block == Block::Grass {
                texture[row as usize][col as usize] = GRASS;
                continue;
            }
            let texture_coords = (coords.0 * 4 + col, coords.1 * 4 + row);
            let seeded_random: u64 = hash(&texture_coords, seed) % pallet.len() as u64;
            texture[row as usize][col as usize] = pallet[seeded_random as usize];
        }
    }
    texture_map.insert(*coords, texture);
    texture
}

fn get_color_pallet(block: Block) -> &'static [Rgb] {
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

pub fn display_view(
    active_chunk: &Chunk,
    player: &Player,
    texture_map: &mut TextureMap,
    seed: &u64,
) {
    print!("\x1b[H");
    for (y, row) in active_chunk.iter().enumerate() {
        let mut textures = [Texture::default(); CHUNK_WIDTH];
        for i in 0..CHUNK_WIDTH {
            textures[i] = get_texture(row[i], texture_map, &(i as i32, y as i32), seed);
        }

        for line in 0..4 {
            for i in 0..CHUNK_WIDTH {
                for (col, color) in textures[i][line].iter().enumerate() {
                    if player.get_x() == i as i32
                        && player.get_y() == y as i32
                        && col == 1
                        && (line == 2 || line == 3)
                    {
                        print!("\x1b[48;2;{};{};{}m██\x1b[0m", color.0, color.1, color.2);
                    } else {
                        print!("\x1b[38;2;{};{};{}m██\x1b[0m", color.0, color.1, color.2);
                    }
                }
            }
            println!();
        }
    }
    if player.is_paused() {
        print!("\x1b[H");

        let sky = format!("\x1b[48;2;{};{};{}m", SKY.0, SKY.1, SKY.2);

        print!("{}", sky);

        println!("\n\n");
        println!("██████████████████████████████");
        println!("██                          ██");
        println!("██          PAUSED          ██");
        println!("██                          ██");
        println!("██████████████████████████████");

        print!("\x1b[0m");
    }
}
