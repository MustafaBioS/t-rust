use std::cmp::PartialEq;
use crate::draw::{Block, Chunk};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use std::io;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub is_grounded: bool,
}

pub fn get_player_position() -> Player {
    let mut player = Player { x: 5, y: 5, is_grounded: true };

    player
}

pub fn move_player(player: &mut Player, state: &Chunk) -> io::Result<bool> {
    if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
        if kind != KeyEventKind::Press {
            return Ok(true);
        }

        if state[player.y as usize][player.x as usize] == Block::Grass
            || state[player.y as usize][player.x as usize] == Block::Dirt  {
            player.is_grounded = true
        } else {
            player.is_grounded = false
        }

        match code {
            KeyCode::Char('d') | KeyCode::Right => {
                let new_x = player.x + 1;

                if state[player.y as usize][new_x as usize] == Block::Air {
                    player.x = new_x;
                }

                if player.is_grounded == false {
                    player.y += 1
                }

            }
            KeyCode::Char('a') | KeyCode::Left => {
                let new_x = player.x - 1;

                if state[player.y as usize][new_x as usize] == Block::Air {
                    player.x = new_x;
                }

                if player.is_grounded == false {
                    player.y += 1
                }

            }
            KeyCode::Char(' ') | KeyCode::Up => {
                if player.is_grounded == true {
                    let new_y = player.y - 1;

                    if state[new_y as usize][player.x as usize] == Block::Air {
                        player.y = new_y;
                    }
                }
            }
            KeyCode::Char('q') => {
                return Ok(false);
            }
            _ => {}
        }
    }

    Ok(true)
}
