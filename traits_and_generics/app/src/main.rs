use rand::Rng;

use ggez::conf;

use core_lib::{self, GameState, Move};
use secondary_lib::HumanPlayer;
use tertiary_lib::AiPlayer;

fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Test Game", "Rufus Stone")
        .window_setup(conf::WindowSetup::default().title("Test Game").vsync(true))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()
        .unwrap();

    let mut prng = rand::thread_rng();

    let coin_toss: bool = prng.gen();

    let player: Box<dyn Move> = match coin_toss {
        true => {
            println!("Starting a regular game");
            Box::new(HumanPlayer::default())
        }
        false => {
            println!("Starting an AI game");
            Box::new(AiPlayer::default())
        }
    };

    let game_state = GameState::new(player, prng);

    ggez::event::run(ctx, event_loop, game_state);

    // This also works
    /*match coin_toss {
        true => {
            println!("Starting a regular game");
            let human_player = HumanPlayer::default();
            let game_state = GameState::new(human_player, prng);

            ggez::event::run(ctx, event_loop, game_state);
        }
        false => {
            println!("Starting an AI game");
            let ai_player = AiPlayer::default();
            let game_state = GameState::new(ai_player, prng);

            ggez::event::run(ctx, event_loop, game_state);
        }
    }*/
}
