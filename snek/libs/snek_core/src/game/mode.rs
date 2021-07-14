use crate::players::*;

#[derive(Debug)]
pub enum Mode {
    OnePlayer(Player),
    TrainAi(Player),
}
