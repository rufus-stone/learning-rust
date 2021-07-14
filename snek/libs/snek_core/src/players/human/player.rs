use ggez::input::keyboard;

use crate::{
    game::{
        snek::{LEFT, RIGHT},
        types::Vec2,
    },
    players::Move,
};

#[derive(Debug)]
pub struct Controls {
    left: keyboard::KeyCode,
    right: keyboard::KeyCode,
}

#[derive(Debug)]
pub struct HumanPlayer {
    controls: Controls,
}

impl HumanPlayer {
    pub fn new(left: keyboard::KeyCode, right: keyboard::KeyCode) -> HumanPlayer {
        let player = HumanPlayer {
            controls: Controls { left, right },
        };

        log::info!("New human player: {:?}", &player);

        player
    }
}

impl Default for HumanPlayer {
    fn default() -> Self {
        HumanPlayer {
            controls: Controls {
                left: keyboard::KeyCode::Left,
                right: keyboard::KeyCode::Right,
            },
        }
    }
}

impl Move for HumanPlayer {
    fn make_move(&self, _ctx: &mut ggez::Context) -> Option<Vec2> {
        // Check for key presses and return a new direction for the Snek accordingly
        if keyboard::is_key_pressed(_ctx, self.controls.left) {
            Some(LEFT)
        } else if keyboard::is_key_pressed(_ctx, self.controls.right) {
            Some(RIGHT)
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
        let player = HumanPlayer::new(keyboard::KeyCode::A, keyboard::KeyCode::D);
        assert_eq!(player.controls.left, keyboard::KeyCode::A);
        assert_eq!(player.controls.right, keyboard::KeyCode::D);
    }

    #[test]
    fn default_human_player() {
        let player = HumanPlayer::default();
        assert_eq!(player.controls.left, keyboard::KeyCode::Left);
        assert_eq!(player.controls.right, keyboard::KeyCode::Right);
    }
}
