use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod game;

use game::food::Food;
use game::snek::Snek;
use game::types::Vec2;

fn main() {
    let mut prng = ChaCha8Rng::from_seed(Default::default());
    //let mut prng = rand::thread_rng();

    // Set the bounds of the game arena
    let bounds = Vec2::new(10, 10);

    // Snek always starts at position 0,0
    let mut snek = Snek::default();

    // Pick a random spot for the food
    let mut food = Food::random(&bounds, &snek, &mut prng);

    println!("{:?}", &food);
    println!("{:?}", &snek);
    println!("-----------------");

    // Make some moves
    for step in 0..1000 {
        if snek.advance(&bounds, food.pos()) {
            food = Food::random(&bounds, &snek, &mut prng);
        }

        // Check if the Snek hit itself
        if snek.hit_self() {
            println!("{}: Snek hit itself! {:?}", step, &snek);
            break;
        }

        snek.random_direction(&mut prng);

        println!("{}: {:?} -- {:?}", step, &food, &snek);
    }

    println!("FINAL SCORE: {}", snek.len());
}
