use ggez::input::keyboard;

use crate::entities::snek::{FACING_DOWN, FACING_LEFT, FACING_RIGHT, FACING_UP};
use crate::players::Move;
use crate::types::Vec2;

#[derive(Debug)]
pub struct Controls {
    left: keyboard::KeyCode,
    right: keyboard::KeyCode,
    up: keyboard::KeyCode,
    down: keyboard::KeyCode,
}

#[derive(Debug)]
pub struct HumanPlayer {
    controls: Controls,
}

impl HumanPlayer {
    pub fn new(
        left: keyboard::KeyCode,
        right: keyboard::KeyCode,
        up: keyboard::KeyCode,
        down: keyboard::KeyCode,
    ) -> HumanPlayer {
        let player = HumanPlayer {
            controls: Controls {
                left,
                right,
                up,
                down,
            },
        };

        log::warn!("New HumanPlayer: {:?}", &player);

        player
    }
}

impl Default for HumanPlayer {
    fn default() -> Self {
        let player = HumanPlayer {
            controls: Controls {
                left: keyboard::KeyCode::Left,
                right: keyboard::KeyCode::Right,
                up: keyboard::KeyCode::Up,
                down: keyboard::KeyCode::Down,
            },
        };

        log::warn!("New default HumanPlayer: {:?}", &player);

        player
    }
}

impl Move for HumanPlayer {
    fn make_move(&self, _ctx: &mut ggez::Context) -> Option<Vec2> {
        // Check for key presses and return a new direction for the Snek accordingly
        if keyboard::is_key_pressed(_ctx, self.controls.left) {
            Some(FACING_LEFT)
        } else if keyboard::is_key_pressed(_ctx, self.controls.right) {
            Some(FACING_RIGHT)
        } else if keyboard::is_key_pressed(_ctx, self.controls.up) {
            Some(FACING_UP)
        } else if keyboard::is_key_pressed(_ctx, self.controls.down) {
            Some(FACING_DOWN)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_human_player() {
        let player = HumanPlayer::new(
            keyboard::KeyCode::A,
            keyboard::KeyCode::D,
            keyboard::KeyCode::W,
            keyboard::KeyCode::S,
        );
        assert_eq!(player.controls.left, keyboard::KeyCode::A);
        assert_eq!(player.controls.right, keyboard::KeyCode::D);
        assert_eq!(player.controls.up, keyboard::KeyCode::W);
        assert_eq!(player.controls.down, keyboard::KeyCode::S);
    }

    #[test]
    fn default_human_player() {
        let player = HumanPlayer::default();
        assert_eq!(player.controls.left, keyboard::KeyCode::Left);
        assert_eq!(player.controls.right, keyboard::KeyCode::Right);
        assert_eq!(player.controls.up, keyboard::KeyCode::Up);
        assert_eq!(player.controls.down, keyboard::KeyCode::Down);
    }
}
