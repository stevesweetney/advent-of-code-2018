use std::{
    char::ParseCharError,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

const PLAYERS: usize = 426;
const LAST_MARBLE_VALUE: usize = 72058;

#[aoc(day9, part1)]
fn solve_part1(_: &str) -> usize {
    winning_score(PLAYERS, LAST_MARBLE_VALUE)
}

fn winning_score(num_players: usize, last_marble: usize) -> usize {
    let mut player_scores = HashMap::new();
    let mut game_board = vec![0, 1];
    let (mut curr_marble, mut player): (usize, usize) = (1,1);

    for m in 2..=last_marble {
        match m % 23 {
            0 => {
                let player_score = player_scores.entry(player).or_insert(0);
                *player_score += m;
                curr_marble = ((curr_marble + game_board.len()).wrapping_sub(7)) % game_board.len();
                *player_score += game_board.remove(curr_marble);
                curr_marble %= game_board.len();
            },
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winning_score() {
        assert_eq!(winning_score(10, 1618), 8317);
        assert_eq!(winning_score(13, 7999), 146373);
        assert_eq!(winning_score(17, 1104), 2764);
        assert_eq!(winning_score(21, 6111), 54718);
        assert_eq!(winning_score(30, 5807), 37305);
    }
}
