use crate::draw::{Block, Chunk};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use std::io;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub is_grounded: bool,
    pub is_paused: bool,
}

impl Player {
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn move_player(player: &mut Player, state: &Chunk) -> io::Result<bool> {
    if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
        if kind != KeyEventKind::Press {
            return Ok(true);
        }

        let below_y = (player.y + 1) as usize;
        let left_x = (player.x - 1) as usize;
        let right_x = (player.x + 1) as usize;
        let center_x = player.x as usize;

        if state[below_y][left_x] != Block::Air
            || state[below_y][center_x] != Block::Air
            || state[below_y][right_x] != Block::Air
        {
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
            KeyCode::Char('p') | KeyCode::Esc => {
                player.is_paused = !player.is_paused;
            }
            KeyCode::Char('q') => {
                return Ok(false);
            }
            _ => {}
        }
    }

    Ok(true)
}
