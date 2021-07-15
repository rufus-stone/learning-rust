use rand::{prelude::ThreadRng, Rng, RngCore};

use core_lib::Move;

pub struct Brain<R: RngCore> {
    prng: R,
}

impl<R: RngCore> Brain<R> {
    pub fn new(prng: R) -> Self {
        Self { prng }
    }
}

impl Default for Brain<ThreadRng> {
    fn default() -> Self {
        Self {
            prng: rand::thread_rng(),
        }
    }
}

pub struct AiPlayer<R: RngCore> {
    brain: Brain<R>,
}

impl Default for AiPlayer<ThreadRng> {
    fn default() -> Self {
        Self {
            brain: Brain::default(),
        }
    }
}

impl<R: RngCore> AiPlayer<R> {
    pub fn new(prng: R) -> Self {
        Self {
            brain: Brain::new(prng),
        }
    }
}

impl<R: RngCore> Move for AiPlayer<R> {
    fn make_move(&mut self) -> u8 {
        2 + self.brain.prng.gen_range(0..=2)
    }
}
