mod draw;
mod generator;
mod player;

use crate::generator::WorldContent;
use crate::player::move_player;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use draw::display_view;
use std::io;

fn main() -> io::Result<()> {
    let mut test = false;
    let mut game_state = WorldContent::default();
    game_state.generate_seed();
    game_state.player_position.set_position(5, 5);
    game_state.generate_initial_chunk();

    println!(" ");
    println!("Main Menu");
    println!(" ");
    println!("1 - Start");
    println!("2 - Exit");
    println!(" ");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

    let trimmed = inp.trim();

    if trimmed == "1" {
        test = true;
    } else if trimmed == "2" {
        test = false;
    } else {
        println!("Invalid Choice");
    }


    if test == true {
        enable_raw_mode()?;

        loop {
            display_view(
                &game_state.rendered_chunks[1],
                &game_state.player_position,
                &mut game_state.texture_map,
                &game_state.seed,
            );
            if !move_player(
                &mut game_state.player_position,
                &game_state.rendered_chunks[1],
            )? {
                break;
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
