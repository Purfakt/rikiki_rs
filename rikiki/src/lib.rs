use round::{Bet, BettingPhase, FinishedRound, NewRound, Points, ScoringPhase};

pub mod round;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Player(pub String);

impl<T: Into<String>> From<T> for Player {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

#[derive(Debug, Clone)]
pub enum CurrentRound {
    NewRound(NewRound),
    BettingPhase(BettingPhase),
    ScoringPhase(ScoringPhase),
    FinishedRound(FinishedRound),
}

pub struct Game {
    amount_of_rounds: usize,
    rounds: Vec<FinishedRound>,
}
pub struct NewGame;

pub struct GameBetting {
    game: Game,
    current_round: BettingPhase,
}

pub struct GameScoring {
    game: Game,
    current_round: ScoringPhase,
}

pub struct GameRoundFinished {
    game: Game,
    current_round: FinishedRound,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let amount_players = players.len();
        let amount_of_cards: usize = std::cmp::min(10, 52 / amount_players);
        let amount_of_rounds: usize = amount_of_cards * 2;

        Self {
            amount_of_rounds,
            rounds: Vec::with_capacity(amount_of_rounds),
        }
    }

    pub fn get_scores(&self) -> Vec<Vec<i8>> {
        self.rounds.iter().map(|round| round.get_scores()).collect()
    }
}

impl NewGame {
    pub fn with_players(players: Vec<Player>) -> GameBetting {
        let amount_players = players.len();
        let amount_of_cards: usize = std::cmp::min(10, 52 / amount_players);

        let game = Game::new(players);
        let current_round = NewRound::new(0, amount_of_cards, amount_players);
        GameBetting { game, current_round }
    }
}

impl GameBetting {
    pub fn lock_bets(self, bets: Bet) -> GameScoring {
        let current_round = self.current_round.lock_bets(bets);
        GameScoring {
            game: self.game,
            current_round,
        }
    }

    pub fn get_scores(&self) -> Vec<Vec<i8>> {
        self.game.rounds.iter().map(|round| round.get_scores()).collect()
    }
}

impl GameScoring {
    pub fn lock_points(self, points: Points) -> GameRoundFinished {
        let current_round = self.current_round.lock_points(points);
        GameRoundFinished {
            current_round,
            game: self.game,
        }
    }

    pub fn get_scores(&self) -> Vec<Vec<i8>> {
        self.game.rounds.iter().map(|round| round.get_scores()).collect()
    }
}

pub enum NextRoundOrGame {
    NextRound(GameBetting),
    GameOver(Game),
}

impl GameRoundFinished {
    pub fn next_round(mut self) -> NextRoundOrGame {
        let amount_of_rounds = self.game.amount_of_rounds;

        if amount_of_rounds == self.game.rounds.len() {
            return NextRoundOrGame::GameOver(self.game);
        }

        let current_round = NewRound::from_previous(&self.current_round);

        self.game.rounds.push(self.current_round);

        NextRoundOrGame::NextRound(GameBetting {
            game: self.game,
            current_round,
        })
    }

    pub fn get_scores(&self) -> Vec<Vec<i8>> {
        self.game.rounds.iter().map(|round| round.get_scores()).collect()
    }
}
