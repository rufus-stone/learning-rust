trait Player {
    fn make_move(&self) -> u8;
}

#[derive(Default)]
struct HumanPlayer {}

impl Player for HumanPlayer {
    fn make_move(&self) -> u8 {
        1
    }
}

#[derive(Default)]
struct AiPlayer {}

impl Player for AiPlayer {
    fn make_move(&self) -> u8 {
        0
    }
}

struct Game<P: Player> {
    player: P,
}

impl<P: Player> Game<P> {
    fn new(player: P) -> Self {
        Self { player }
    }
}

// We have to specify that Default only works with Game<HumanPlayer>
// If instead we tried `impl<P: Player> Default for Game<P>` then we'd get the error "expected type parameter `P`, found struct `HumanPlayer`" on the line `player: HumanPlayer::default(),`
impl Default for Game<HumanPlayer> {
    fn default() -> Self {
        Self {
            player: HumanPlayer::default(),
        }
    }
}

fn main() {
    let game = Game::new(HumanPlayer::default());
    let game = Game::new(AiPlayer {});
    let game = Game::default();
}

#[test]
fn game() {
    let game = Game::new(HumanPlayer::default());
    assert_eq!(game.player.make_move(), 1);

    let game = Game::new(AiPlayer {});
    assert_eq!(game.player.make_move(), 0);

    let game = Game::default();
    assert_eq!(game.player.make_move(), 1);
}
