use regex::Regex;
use std::{char::ParseCharError, str::FromStr};

lazy_static! {
    static ref regexep: Regex = Regex::new(
        r"(?x)
        (?P<two_left>\#|\.)(?P<left>\#|\.)(?P<curr>\#|\.)
        (?P<right>\#|\.)(?P<two_right>\#|\.)\s+ => \s+ (?P<next>\#|\.)
    "
    )
    .unwrap();
}

const MAX_PLANTS: usize = 1001;

fn input_gen(input: &str) -> Plants {
    let mut lines = input.lines();
    let mut initial_state = vec![false; MAX_PLANTS];
    if let Some(l) = lines.next() {
        let state = l.trim_start_matches("initial state:").trim();
        let start = MAX_PLANTS / 2;
        for (c, i) in state.chars().zip(start..) {
            initial_state[i] = has_plant(c);
        }
    }
    lines.next();

    let rules: Vec<_> = lines.map(|l| l.parse::<Rule>().unwrap()).collect();

    Plants {
        state: initial_state,
        rules,
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut p = input_gen(input);

    for _ in 0..20 {
        p.next_gen();
    }

    p.sum()
}

fn has_plant(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => {
            panic!("Unknown plant type");
        }
    }
}

#[derive(Debug, Clone)]
struct Plants {
    state: Vec<bool>,
    rules: Vec<Rule>,
}

impl Plants {
    fn next_gen(&mut self) {
        let mut next = vec![false; MAX_PLANTS];
        for i in 2..MAX_PLANTS - 2 {
            next[i] = self.has_plant_in_next_gen(i);
        }

        self.state = next;
    }

    fn has_plant_in_next_gen(&self, n: usize) -> bool {
        let plants = &self.state[n - 2..=n + 2];
        for rule in self.rules.iter() {
            if rule.plants == plants {
                return rule.next;
            }
        }
        false
    }

    fn sum(&self) -> i32 {
        let mut sum = 0;
        for (i, plant) in self.state.iter().enumerate() {
            if *plant {
                sum += i as i32 - (MAX_PLANTS / 2) as i32;
            }
        }

        sum
    }
}

#[derive(Debug, Clone)]
struct Rule {
    plants: [bool; 5],
    next: bool,
}

impl FromStr for Rule {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = regexep.captures(s).expect("Error while parsing for Rule");
        let mut plants = [false; 5];
        plants[0] = has_plant(caps["two_left"].parse()?);
        plants[1] = has_plant(caps["left"].parse()?);
        plants[2] = has_plant(caps["curr"].parse()?);
        plants[3] = has_plant(caps["right"].parse()?);
        plants[4] = has_plant(caps["two_right"].parse()?);
        Ok(Rule {
            plants,
            next: has_plant(caps["next"].parse()?),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/tests/d12.txt");
        let sum = solve_part1(input);

        assert_eq!(sum, 325);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input/tests/d12.txt");
        let plants = input_gen(input);

        println!("{:?}", plants);

        assert_eq!(plants.rules.len(), 14);
    }
}
