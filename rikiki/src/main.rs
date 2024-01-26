use rikiki::{NewGame, Player};

fn main() {
    let mut game = NewGame::with_players(vec![
        Player("Alice".to_string()),
        Player("Bob".to_string()),
        Player("Charlie".to_string()),
        Player("Diana".to_string()),
    ]);

    game.add_bet(0, 0);
    game.add_bet(1, 1);
    game.add_bet(2, 2);
    game.add_bet(3, 3);

    let mut game = game.lock_bets().unwrap();

    game.add_points(0, 0);
    game.add_points(1, 1);
    game.add_points(2, 2);
    game.add_points(3, 3);

    let _game = game.lock_points().unwrap();
}
