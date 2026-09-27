use crate::draw::{Block, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::generator::WorldContent;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use std::cmp::{max, min};
use std::io;

pub struct Player {
    x: i32,
    y: i32,
    is_grounded: bool,
    is_paused: bool,
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
            is_grounded: false,
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

    fn shift_x_by(&mut self, amount: i32) {
        self.x += amount;
        self.clamp();
    }

    fn shift_y_by(&mut self, amount: i32) {
        self.y += amount;
        self.clamp();
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    fn get_shift(&self, direction: Direction) -> usize {
        match direction {
            Direction::Up => max(self.y - 1, 0) as usize,
            Direction::Down => min((self.y + 1), CHUNK_HEIGHT as i32 - 1) as usize,
            Direction::Left => max(self.x - 1, 0) as usize,
            Direction::Right => min((self.x + 1), CHUNK_WIDTH as i32 - 1) as usize,
        }
    }

    pub fn is_clear(&self, direction: Direction, game_state: &WorldContent) -> bool {
        match direction {
            Direction::Up => {
                game_state.state[self.get_shift(Direction::Up)][self.get_x() as usize] == Block::Air
            }
            Direction::Down => {
                game_state.state[self.get_shift(Direction::Down)][self.get_x() as usize]
                    == Block::Air
            }
            Direction::Left => {
                game_state.state[self.get_y() as usize][self.get_shift(Direction::Left)]
                    == Block::Air
            }
            Direction::Right => {
                game_state.state[self.get_y() as usize][self.get_shift(Direction::Right)]
                    == Block::Air
            }
        }
    }

    pub fn mv(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.shift_y_by(-1),
            Direction::Down => self.shift_y_by(1),
            Direction::Left => self.shift_x_by(-1),
            Direction::Right => self.shift_x_by(1),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn set_ground(&mut self, value: bool) {
        self.is_grounded = value;
    }
    pub fn is_grounded(&self, game_state: &WorldContent) -> bool {
        game_state.state[self.get_shift(Direction::Down)][self.get_shift(Direction::Left)]
            != Block::Air
            || game_state.state[self.get_shift(Direction::Down)][self.get_x() as usize]
                != Block::Air
            || game_state.state[self.get_shift(Direction::Down)][self.get_shift(Direction::Right)]
                != Block::Air
    }
}

pub fn move_player(game_state: &mut WorldContent) -> io::Result<bool> {
    if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
        if kind != KeyEventKind::Press {
            return Ok(true);
        }

        if game_state.player.is_grounded(&game_state) {
            game_state.player.set_ground(true);
        } else {
            game_state.player.set_ground(false);
        }

        match code {
            KeyCode::Char('d') | KeyCode::Right if !game_state.player.is_paused() => {
                if game_state.player.is_clear(Direction::Right, game_state) {
                    game_state.player.mv(Direction::Right);
                }

                if game_state.player.is_grounded == false {
                    game_state.player.mv(Direction::Down);
                }
            }
            KeyCode::Char('a') | KeyCode::Left if !game_state.player.is_paused() => {
                if game_state.player.is_clear(Direction::Left, game_state) {
                    game_state.player.mv(Direction::Left);
                }

                if game_state.player.is_grounded == false {
                    game_state.player.mv(Direction::Down);
                }
            }
            KeyCode::Char(' ') | KeyCode::Up if !game_state.player.is_paused => {
                if game_state.player.is_grounded == true {
                    if game_state.player.is_clear(Direction::Up, game_state) {
                        game_state.player.mv(Direction::Up);
                    }
                }
            }
            KeyCode::Char('p') | KeyCode::Esc => {
                game_state.player.is_paused = !game_state.player.is_paused;
            }
            KeyCode::Char('q') => {
                return Ok(false);
            }
            _ => {}
        }
    }

    Ok(true)
}
