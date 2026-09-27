mod draw;
mod generator;
mod player;

use crate::generator::WorldContent;
use crate::player::move_player;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use draw::Block::Air as A;
use draw::Block::Cloud as C;
use draw::Block::Dirt as D;
use draw::Block::Grass as G;
use draw::Block::Stone as S;
use draw::Block::TreeLeaf as L;
use draw::Block::Wood as W;
use draw::Chunk;
use draw::display_view;
use rand::Rng;
use std::io;

fn main() -> io::Result<()> {
    let mut game_state = WorldContent::default();
    game_state.generate_seed();
    game_state.player_position.set_position(5, 5);

    let state: Chunk = [
        [A, A, A, A, A, A, A, A, A, A, A, C, C, C, A, A, A, A],
        [A, A, A, L, L, L, A, A, A, A, C, C, C, C, C, A, A, A],
        [A, A, L, L, L, L, L, A, A, A, A, A, A, A, A, A, A, A],
        [A, A, L, L, L, L, L, A, A, A, A, A, A, A, A, A, G, G],
        [A, A, A, A, W, A, A, A, A, A, A, A, A, A, A, G, D, D],
        [A, A, A, A, W, A, A, A, A, A, A, A, G, G, G, D, D, D],
        [G, G, G, G, G, G, G, G, G, G, G, G, D, D, D, D, S, D],
        [D, D, D, D, D, D, D, D, D, D, D, D, D, S, D, S, S, S],
        [D, D, S, D, D, D, S, D, D, S, S, D, S, S, S, S, S, S],
        [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S],
    ];

    enable_raw_mode();

    loop {
        display_view(
            &state,
            &game_state.player_position,
            &mut game_state.texture_map,
            &game_state.seed,
        );
        if !move_player(&mut game_state.player_position, &state)? {
            break;
        }
    }

    disable_raw_mode();

    Ok(())
}
