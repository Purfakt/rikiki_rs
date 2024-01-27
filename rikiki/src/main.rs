use rikiki::{
    round::{Bet, Points},
    NewGame, NextRoundOrGame, Player,
};

fn main() {
    let game = NewGame::with_players(vec![
        Player("Alice".to_string()),
        Player("Bob".to_string()),
        Player("Charlie".to_string()),
        Player("Diana".to_string()),
    ]);

    let game = game.lock_bets(Bet::from(vec![0, 1, 2, 3]));
    let game = game.lock_points(Points::from(vec![0, 1, 2, 3]));

    match game.next_round() {
        NextRoundOrGame::NextRound(game) => {
            let game = game;
            let scores = game.get_scores();
            println!("Scores: {:?}", scores);

            let game = game.lock_bets(Bet::from(vec![0, 0, 2, 0]));
            let game = game.lock_points(Points::from(vec![0, 5, 2, 3]));

            match game.next_round() {
                NextRoundOrGame::NextRound(game) => {
                    let game = game;
                    let scores = game.get_scores();
                    println!("Scores: {:?}", scores);
                }
                NextRoundOrGame::GameOver(game) => {
                    let scores = game.get_scores();
                    println!("Scores: {:?}", scores);
                }
            };
        }
        NextRoundOrGame::GameOver(game) => {
            let scores = game.get_scores();
            println!("Scores: {:?}", scores);
        }
    };
}
