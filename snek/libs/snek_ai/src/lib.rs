use snek_core::{players::Move, types::Vec2};

pub struct AiPlayer {}

impl Default for AiPlayer {
    fn default() -> Self {
        todo!()
    }
}

impl Move for AiPlayer {
    fn make_move(&self, _ctx: &mut ggez::Context) -> Option<Vec2> {
        todo!()
    }
}
