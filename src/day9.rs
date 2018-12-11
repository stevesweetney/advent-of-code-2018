use std::{
    char::ParseCharError,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

const PLAYERS: usize = 426;
const LAST_MARBLE_VALUE: usize = 72058;

#[aoc(day9, part1)]
fn solve_part1(_: &str) -> usize {
    winning_score_fast(PLAYERS, LAST_MARBLE_VALUE)
}

#[aoc(day9, part2)]
fn solve_part2(_: &str) -> usize {
    winning_score_fast(PLAYERS, LAST_MARBLE_VALUE * 100)
}

// This was way too inefficient for part 2 (Ran for over 10 minutes without returning an answer!)
// Arbitrary inserts and removals with arrays is not ideal
fn winning_score(num_players: usize, last_marble: usize) -> usize {
    let mut player_scores = HashMap::new();
    let mut game_board = vec![0, 1];
    let (mut curr_marble, mut player): (usize, usize) = (1, 1);

    for m in 2..=last_marble {
        match m % 23 {
            0 => {
                let player_score = player_scores.entry(player).or_insert(0);
                *player_score += m;
                curr_marble = ((curr_marble + game_board.len()).wrapping_sub(7)) % game_board.len();
                *player_score += game_board.remove(curr_marble);
                curr_marble %= game_board.len();
            }
            _ => {
                curr_marble = (curr_marble + 2) % game_board.len();
                if curr_marble == 0 {
                    game_board.push(m);
                    curr_marble = game_board.len() - 1;
                } else {
                    game_board.insert(curr_marble, m);
                }
            }
        }

        player += 1;
        player %= num_players;
    }

    *player_scores.iter().map(|(p, score)| score).max().unwrap()
}

// This passes all but one test case but gave the correct answer for pt 1 & 2 ...
fn winning_score_fast(num_players: usize, last_marble: usize) -> usize {
    let mut game = Game::new(num_players);

    for m in 2..last_marble {
        game.turn(m);
    }

    *game
        .player_scores
        .iter()
        .map(|(p, score)| score)
        .max()
        .unwrap()
}

struct Game {
    marbles: Vec<Marble>,
    curr_marble: usize,
    max_players: usize,
    curr_player: usize,
    player_scores: HashMap<usize, usize>,
}

impl Game {
    fn new(max_players: usize) -> Self {
        let marbles = vec![
            Marble {
                value: 0,
                ccw: 1,
                cw: 1,
            },
            Marble {
                value: 1,
                ccw: 0,
                cw: 0,
            },
        ];

        Game {
            marbles,
            curr_marble: 1,
            max_players,
            curr_player: 1,
            player_scores: HashMap::new(),
        }
    }

    fn turn(&mut self, m: usize) {
        match m % 23 {
            0 => {
                let mut score = 0;
                score += m;
                let marble_to_remove = self.ccw_from(self.curr_marble, 7);
                let ccw = self.marbles[marble_to_remove].ccw;
                score += self.marbles[marble_to_remove].value;
                self.curr_marble = self.marbles[marble_to_remove].cw;
                assert_eq!(self.marbles[ccw].cw, marble_to_remove);
                self.marbles[ccw].cw = self.curr_marble;
                assert_eq!(self.marbles[self.curr_marble].ccw, marble_to_remove);
                self.marbles[self.curr_marble].ccw = ccw;
                *self.player_scores.entry(self.curr_player).or_insert(0) += score;
            }
            _ => {
                let mut new_m = Marble::new(m);
                let new_m_index = self.marbles.len();
                let before = self.cw_from(self.curr_marble, 1);
                let after = self.cw_from(self.curr_marble, 2);
                new_m.ccw = before;
                new_m.cw = after;
                self.marbles.push(new_m);
                assert_eq!(self.marbles[before].cw, after);
                self.marbles[before].cw = new_m_index;
                assert_eq!(self.marbles[after].ccw, before);
                self.marbles[after].ccw = new_m_index;
                self.curr_marble = new_m_index;
                assert_eq!(self.curr_marble + 1, self.marbles.len());
            }
        }

        self.curr_player += 1;
        self.curr_player %= self.max_players;
    }

    fn cw_from(&self, from: usize, i: usize) -> usize {
        let mut curr = from;
        for _ in 0..i {
            curr = self.marbles[curr].cw;
        }

        curr
    }

    fn ccw_from(&self, from: usize, i: usize) -> usize {
        let mut curr = from;
        for _ in 0..i {
            curr = self.marbles[curr].ccw;
        }

        curr
    }
}

struct Marble {
    value: usize,
    cw: usize,
    ccw: usize,
}

impl Marble {
    fn new(value: usize) -> Self {
        Marble {
            value,
            cw: 0,
            ccw: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winning_score() {
        assert_eq!(winning_score_fast(10, 1618), 8317);
        assert_eq!(winning_score_fast(13, 7999), 146373);
        assert_eq!(winning_score_fast(17, 1104), 2764);
        assert_eq!(winning_score_fast(21, 6111), 54718);
        assert_eq!(winning_score_fast(30, 5807), 37305);
    }
}
