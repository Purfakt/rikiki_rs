#[derive(Debug, Clone)]
pub struct Round<B, P, S> {
    pub context: Context,
    pub bets: B,
    pub points: P,
    pub scores: S,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub dealer_index: usize,
    pub amount_of_cards: usize,
    pub amount_of_players: usize,
    pub incrementing_phase: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NoBet;

#[derive(Debug, Clone, Default)]
pub struct Bet(Vec<i8>);

#[derive(Debug, Clone, Default)]
pub struct NoPoints;

#[derive(Debug, Clone, Default)]
pub struct Points(Vec<i8>);

#[derive(Debug, Clone, Default)]
pub struct NoScores;

#[derive(Debug, Clone, Default)]
pub struct Scores(Vec<i8>);

pub type NewRound = Round<NoBet, NoPoints, NoScores>;
pub type BettingPhase = Round<NoBet, NoPoints, NoScores>;
pub type ScoringPhase = Round<Bet, NoPoints, NoScores>;
pub type FinishedRound = Round<Bet, Points, Scores>;

impl Bet {
    pub fn from(bets: Vec<i8>) -> Self {
        Self(bets)
    }
}

impl Points {
    pub fn from(bets: Vec<i8>) -> Self {
        Self(bets)
    }
}

impl NewRound {
    pub fn new(dealer_index: usize, amount_of_cards: usize, amount_of_players: usize) -> BettingPhase {
        BettingPhase {
            context: Context::new(dealer_index, amount_of_cards, amount_of_players),
            bets: NoBet,
            points: NoPoints,
            scores: NoScores,
        }
    }

    pub fn from_previous(previous: &FinishedRound) -> BettingPhase {
        BettingPhase {
            context: Context::from_previous(&previous.context),
            bets: NoBet,
            points: NoPoints,
            scores: NoScores,
        }
    }
}

impl Context {
    pub fn new(dealer_index: usize, amount_of_cards: usize, amount_of_players: usize) -> Self {
        Self {
            dealer_index,
            amount_of_cards,
            amount_of_players,
            incrementing_phase: false,
        }
    }

    pub fn from_previous(previous: &Context) -> Self {
        let incrementing_phase = previous.incrementing_phase || previous.amount_of_cards == 1;

        let amount_of_cards = if previous.incrementing_phase {
            previous.amount_of_cards + 1
        } else if incrementing_phase {
            1 // We just start the incrementing phase with 1 card
        } else {
            previous.amount_of_cards - 1
        };

        let dealer_index = previous.next_dealer();
        let amount_of_players = previous.amount_of_players;

        Self {
            dealer_index,
            amount_of_cards,
            amount_of_players,
            incrementing_phase,
        }
    }

    fn next_dealer(&self) -> usize {
        (self.dealer_index + 1) % self.amount_of_players
    }
}

impl BettingPhase {
    pub fn lock_bets(self, bets: Bet) -> ScoringPhase {
        ScoringPhase {
            context: self.context,
            bets,
            points: NoPoints,
            scores: NoScores,
        }
    }
}

impl ScoringPhase {
    pub fn lock_points(self, points: Points) -> FinishedRound {
        let scores: Vec<i8> = self
            .bets
            .0
            .iter()
            .zip(points.0.iter())
            .map(|(bet, points)| compute_score(*bet, *points))
            .collect();
        FinishedRound {
            context: self.context,
            bets: self.bets.clone(),
            points,
            scores: Scores(scores),
        }
    }
}

impl FinishedRound {
    pub fn get_scores(&self) -> Vec<i8> {
        self.scores.0.clone()
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
