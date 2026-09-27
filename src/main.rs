mod chest;
mod draw;
mod generator;
pub mod menu;
mod player;

use crate::generator::WorldContent;
use crate::menu::menu;
use crate::player::move_player;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute, terminal::SetSize};
use draw::display_view;
use std::io;
use std::io::stdout;

fn main() -> io::Result<()> {
    menu();
    let mut game_state = WorldContent::default();
    game_state.initialize_state();
    game_state.player.set_position(5, 0);

    execute!(stdout(), SetSize(145, 40))?;
    execute!(stdout(), cursor::Hide)?;
    enable_raw_mode()?;

    loop {
        display_view(
            &game_state.state,
            &game_state.player,
            &mut game_state.texture_map,
            &game_state.seed,
        );
        if !move_player(&mut game_state)? {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
