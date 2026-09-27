use crate::draw::{Block, CHUNK_HEIGHT, CHUNK_WIDTH, Chunk};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use std::io;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub is_grounded: bool,
    pub is_paused: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            is_grounded: true,
            is_paused: false,
        }
    }
}

impl Player {
    fn clamp(&mut self) {
        self.x = self.x.clamp(0, CHUNK_WIDTH as i32 - 1);
        self.y = self.y.clamp(0, CHUNK_HEIGHT as i32 - 1);
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn shift_x_by(&mut self, amount: i32) {
        self.x += amount;
        self.clamp();
    }

    pub fn shift_y_by(&mut self, amount: i32) {
        self.y += amount;
        self.clamp();
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_shift(&self, direction: Direction) -> usize {
        match direction {
            Direction::Up => (self.y - 1) as usize,
            Direction::Down => (self.y + 1) as usize,
            Direction::Left => (self.x - 1) as usize,
            Direction::Right => (self.x + 1) as usize,
        }
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
            KeyCode::Char('d') | KeyCode::Right if !player.is_paused => {
                if state[player.y as usize][player.get_shift(Direction::Right)] == Block::Air {
                    player.shift_x_by(1);
                }

                if player.is_grounded == false {
                    player.shift_y_by(1);
                }
            }
            KeyCode::Char('a') | KeyCode::Left if !player.is_paused => {
                if state[player.y as usize][player.get_shift(Direction::Left)] == Block::Air {
                    player.shift_x_by(-1);
                }

                if player.is_grounded == false {
                    player.shift_y_by(1);
                }
            }
            KeyCode::Char(' ') | KeyCode::Up if !player.is_paused => {
                if player.is_grounded == true {
                    if state[player.get_shift(Direction::Up)][player.x as usize] == Block::Air {
                        player.shift_y_by(-1);
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
