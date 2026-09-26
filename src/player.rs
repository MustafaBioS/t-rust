use crossterm::event::{read, Event, KeyCode, KeyEvent};
use std::io;

pub struct Player {
    pub x: i32,
    pub y: i32
}

pub fn get_player_position() -> Player {
    let mut player = Player { x: 6, y: 5 };

    player
}

pub fn move_player(player: &mut Player) -> io::Result<bool> {

     if let Event::Key(KeyEvent { code, .. }) = read()? {
        match code {
            KeyCode::Char('d') | KeyCode::Right => {
                player.x += 1;
            }
            KeyCode::Char('a') | KeyCode::Left => {
                player.x -= 1;
            }
            KeyCode::Char(' ') | KeyCode::Up => {
                player.y += 1;
            }
            KeyCode::Char('q') => {
                return Ok(false);
            }
            _ => {}
        }
    }

    Ok(true)
}