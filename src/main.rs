mod draw;

mod player;

use draw::Chunk;
use draw::display_view;
use draw::Block::Air as A;
use draw::Block::Cloud as C;
use draw::Block::Grass as G;
use draw::Block::Dirt as D;
use draw::Block::Stone as S;
use draw::Block::Wood as W;
use draw::Block::TreeLeaf as L;
use crate::player::{get_player_position, move_player};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;


fn main() -> io::Result<()> {

let state: Chunk = [
        [A, A, A, A, A, A, A, A, A, A, A, C, C, C, A, A, A, A,],
        [A, A, A, L, L, L, A, A, A, A, C, C, C, C, C, A, A, A,],
        [A, A, L, L, L, L, L, A, A, A, A, A, A, A, A, A, A, A,],
        [A, A, L, L, L, L, L, A, A, A, A, A, A, A, A, A, G, G,],
        [A, A, A, A, W, A, A, A, A, A, A, A, A, A, A, G, D, D,],
        [A, A, A, A, W, A, A, A, A, A, A, A, G, G, G, D, D, D,],
        [G, G, G, G, G, G, G, G, G, G, G, G, D, D, D, D, S, D,],
        [D, D, D, D, D, D, D, D, D, D, D, D, D, S, D, S, S, S,],
        [D, D, S, D, D, D, S, D, D, S, S, D, S, S, S, S, S, S,],
        [S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S,],
    ];
    let mut player = get_player_position();

    enable_raw_mode();

    loop {
        display_view(&state, &player);
        if !move_player(&mut player, &state)? {
            break;
        }
    }

    disable_raw_mode();

    Ok(())

}