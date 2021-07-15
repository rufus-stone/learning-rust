use core_lib::Move;

pub struct Controls {
    left: char,
    right: char,
    up: char,
    down: char,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            left: 'A',
            right: 'D',
            up: 'W',
            down: 'S',
        }
    }
}

pub struct HumanPlayer {
    controls: Controls,
}

impl Default for HumanPlayer {
    fn default() -> Self {
        Self {
            controls: Controls::default(),
        }
    }
}

impl Move for HumanPlayer {
    fn make_move(&mut self) -> u8 {
        42
    }
}
