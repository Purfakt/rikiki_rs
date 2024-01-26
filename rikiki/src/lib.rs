#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Player(pub String);

impl<T: Into<String>> From<T> for Player {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

#[derive(Debug, Clone)]
pub struct Round<B, P, S> {
    dealer_index: usize,
    amount_of_players: usize,
    amount_of_cards: usize,
    bets: B,
    points: P,
    scores: S,
}

#[derive(Debug, Clone, Default)]
pub struct NoBet;

#[derive(Debug, Clone, Default)]
pub struct Bet(Vec<Option<i8>>);
#[derive(Debug, Clone, Default)]
pub struct LockedBet(Vec<i8>);

#[derive(Debug, Clone, Default)]
pub struct NoPoints;

#[derive(Debug, Clone, Default)]
pub struct Points(Vec<Option<i8>>);

#[derive(Debug, Clone, Default)]
pub struct LockedPoints(Vec<i8>);

#[derive(Debug, Clone, Default)]
pub struct NoScores;

#[derive(Debug, Clone, Default)]
pub struct Scores(Vec<i8>);

type NewRound = Round<NoBet, NoPoints, NoScores>;
type BettingPhase = Round<Bet, NoPoints, NoScores>;
type ScoringPhase = Round<LockedBet, Points, NoScores>;
type FinishedRound = Round<LockedBet, LockedPoints, Scores>;

#[derive(Debug, Clone)]
pub enum CurrentRound {
    NewRound(NewRound),
    BettingPhase(BettingPhase),
    ScoringPhase(ScoringPhase),
    FinishedRound(FinishedRound),
}

pub struct Game {
    players: Vec<Player>,
    amount_of_rounds: usize,
    incrementing_cards: bool,
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

impl NewRound {
    pub fn new(dealer_index: usize, amount_of_cards: usize, amount_of_players: usize) -> BettingPhase {
        BettingPhase {
            dealer_index,
            amount_of_cards,
            amount_of_players,
            bets: Bet(vec![None; amount_of_players]),
            points: NoPoints,
            scores: NoScores,
        }
    }
}

impl BettingPhase {
    pub fn add_bet(&mut self, player_index: usize, bet: i8) {
        self.bets.0[player_index] = Some(bet);
    }

    pub fn lock_bets(&mut self) -> Option<ScoringPhase> {
        let locked_bets: Option<Vec<i8>> = self.bets.0.iter().filter_map(|&x| x).map(Some).collect();
        let locked_bets = locked_bets?;

        Some(ScoringPhase {
            dealer_index: self.dealer_index,
            amount_of_cards: self.amount_of_cards,
            amount_of_players: self.amount_of_players,
            bets: LockedBet(locked_bets),
            points: Points(vec![None; self.amount_of_players]),
            scores: NoScores,
        })
    }
}

impl ScoringPhase {
    pub fn add_points(&mut self, player_index: usize, points: i8) {
        self.points.0[player_index] = Some(points);
    }

    pub fn lock_points(&mut self) -> Option<FinishedRound> {
        let locked_points: Option<Vec<i8>> = self.points.0.iter().filter_map(|&x| x).map(Some).collect();
        let locked_points = locked_points?;

        let scores: Vec<i8> = self
            .bets
            .0
            .iter()
            .zip(locked_points.iter())
            .map(|(bet, points)| compute_score(*bet, *points))
            .collect();
        Some(FinishedRound {
            dealer_index: self.dealer_index,
            amount_of_cards: self.amount_of_cards,
            amount_of_players: self.amount_of_players,
            bets: self.bets.clone(),
            points: LockedPoints(locked_points),
            scores: Scores(scores),
        })
    }
}

impl FinishedRound {
    pub fn get_scores(&self) -> Vec<i8> {
        self.scores.0.clone()
    }
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        let amount_players = players.len();
        let amount_of_cards: usize = std::cmp::min(10, 52 / amount_players);
        let amount_of_rounds: usize = amount_of_cards * 2;

        Self {
            players,
            amount_of_rounds,
            rounds: Vec::with_capacity(amount_of_rounds),
            incrementing_cards: false,
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
    pub fn add_bet(&mut self, player_index: usize, bet: i8) {
        self.current_round.add_bet(player_index, bet)
    }

    pub fn lock_bets(mut self) -> Option<GameScoring> {
        let current_round = self.current_round.lock_bets()?;
        Some(GameScoring {
            game: self.game,
            current_round,
        })
    }
}

impl GameScoring {
    pub fn add_points(&mut self, player_index: usize, points: i8) {
        self.current_round.add_points(player_index, points)
    }

    pub fn lock_points(mut self) -> Option<GameRoundFinished> {
        let round = self.current_round.lock_points()?;

        Some(GameRoundFinished {
            current_round: round,
            game: self.game,
        })
    }
}

pub enum NextRoundOrGame {
    NextRound(GameBetting),
    GameOver(Game),
}

impl GameRoundFinished {
    pub fn next_round(mut self) -> NextRoundOrGame {
        let amount_of_rounds = self.game.amount_of_rounds;
        let amount_of_cards = self.current_round.amount_of_cards;
        let incrementing_cards = self.game.incrementing_cards;
        let amount_of_players = self.current_round.amount_of_players;
        let dealer_index = self.next_dealer();

        self.game.rounds.push(self.current_round);

        if amount_of_rounds == self.game.rounds.len() {
            return NextRoundOrGame::GameOver(self.game);
        }

        let amount_of_cards = if incrementing_cards {
            amount_of_cards + 1
        } else if amount_of_cards == 1 {
            self.game.incrementing_cards = true;
            1
        } else {
            amount_of_cards - 1
        };

        let current_round = NewRound::new(dealer_index, amount_of_cards, amount_of_players);

        NextRoundOrGame::NextRound(GameBetting {
            game: self.game,
            current_round,
        })
    }

    fn next_dealer(&self) -> usize {
        (self.current_round.dealer_index + 1) % self.game.players.len()
    }
}

const fn compute_score(bet: i8, points: i8) -> i8 {
    if bet == points {
        bet + 2
    } else if bet < points {
        bet - points
    } else {
        points - bet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 2)]
    #[case(0, 1, -1)]
    #[case(1, 0, -1)]
    #[case(3, 3, 5)]
    #[case(3, 6, -3)]
    #[case(4, 3, -1)]
    fn compute_score_test(#[case] bet: i8, #[case] points: i8, #[case] expected_score: i8) {
        let score = compute_score(bet, points);

        assert_eq!(score, expected_score);
    }
}
