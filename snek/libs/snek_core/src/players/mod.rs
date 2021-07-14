use crate::types::Vec2;

pub mod human;

#[derive(Debug)]
pub enum Player {
    Human,
    Computer,
}

/// All players should implement Move in order to play the game
pub trait Move {
    fn make_move(&self, _ctx: &mut ggez::Context) -> Option<Vec2>;
}

/// Implement `Move` for boxed references to a `Move`
impl<'a, M: Move + ?Sized> Move for &'a mut M {
    fn make_move(&self, _ctx: &mut ggez::Context) -> Option<Vec2> {
        (**self).make_move(_ctx)
    }
}
