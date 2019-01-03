use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::{
    char::ParseCharError,
    cmp::{max, min},
    convert::From,
    str::FromStr,
};

const WATER_SPRING: Coord = Coord { x: 500, y: 0 };

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let mut r: Reservoir = input.parse().unwrap();
    r.run(WATER_SPRING);
    r.count()
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> usize {
    let mut r: Reservoir = input.parse().unwrap();
    r.run(WATER_SPRING);
    r.rest_count()
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum State {
    Sand,
    WetSand, // sand that water has passed through
    Clay,
    RestingWater,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(t: (usize, usize)) -> Self {
        Coord { x: t.0, y: t.1 }
    }
}

struct Reservoir {
    visited: HashSet<Coord>,
    tiles: HashMap<Coord, State>,
    min_y: usize,
    max_y: usize,
}

impl Reservoir {
    // water_start is a source of water like a water spring or the edge from an overflow
    fn run(&mut self, water_start: Coord) {
        // repeatedly drop water from the current position making sure to only scan 
        // new tiles and tiles in range
        loop {
            if let Some(mut c) = self.fall(water_start) {
                if self.visited.contains(&c) {
                    break;
                }
                self.scan(c);
            } else {
                break;
            }
        }
    }

    fn scan(&mut self, c: Coord) {
        self.visited.insert(c);
        let (flow_left, spill_left) = self.flow_left(c);
        let (flow_right, spill_right) = self.flow_right(c);
        match (spill_left, spill_right) {
            // no spill on either side means water can fill this area
            (false, false) => {
                for x in flow_left.x..=flow_right.x {
                    self.tiles
                        .insert((x, flow_left.y).into(), State::RestingWater);
                }
            }
            (fall_left, fall_right) => {
                for x in flow_left.x..=flow_right.x {
                    self.tiles.insert((x, flow_left.y).into(), State::WetSand);
                }
                if fall_left {
                    self.run(flow_left);
                }

                if fall_right {
                    self.run(flow_right);
                }
            }
        }
    }

    fn fall(&mut self, water_start: Coord) -> Option<Coord> {
        let mut curr_water = water_start;
        // pour water downwards until we reach clay or resting water
        while {
            match self.tiles.get(&(curr_water.x, curr_water.y + 1).into()) {
                None | Some(&State::WetSand) | Some(&State::Sand) => true,
                Some(&State::Clay) | Some(&State::RestingWater) => false,
            }
        } {
            curr_water.y += 1;

            if curr_water.y > self.max_y {
                return None;
            }
            self.tiles
                .insert((curr_water.x, curr_water.y).into(), State::WetSand);
        }

        Some(curr_water)
    }

    fn flow_left(&mut self, mut water_start: Coord) -> (Coord, bool) {
        let mut spill = false;
        while {
            match self.tiles.get(&(water_start.x, water_start.y + 1).into()) {
                Some(State::Clay) | Some(State::RestingWater) => true,
                _ => {
                    spill = true;
                    false
                }
            }
        } {
            match self.tiles.get(&(water_start.x - 1, water_start.y).into()) {
                Some(State::Clay) => break,
                _ => water_start.x -= 1,
            }
        }

        (water_start, spill)
    }

    fn flow_right(&mut self, mut water_start: Coord) -> (Coord, bool) {
        let mut spill = false;
        while {
            match self.tiles.get(&(water_start.x, water_start.y + 1).into()) {
                Some(State::Clay) | Some(State::RestingWater) => true,
                _ => {
                    spill = true;
                    false
                }
            }
        } {
            match self.tiles.get(&(water_start.x + 1, water_start.y).into()) {
                Some(State::Clay) => break,
                _ => water_start.x += 1,
            }
        }

        (water_start, spill)
    }

    fn count(&self) -> usize {
        let mut tiles_water_reached = 0;
        for (c, state) in &self.tiles {
            if c.y < self.min_y || c.y > self.max_y {
                continue;
            }
            match state {
                State::WetSand | State::RestingWater => tiles_water_reached += 1,
                _ => (),
            }
        }

        tiles_water_reached
    }

    fn rest_count(&self) -> usize {
        let mut resting_water = 0;
        for (c, state) in &self.tiles {
            if c.y < self.min_y || c.y > self.max_y {
                continue;
            }
            match state {
                State::RestingWater => resting_water += 1,
                _ => (),
            }
        }

        resting_water
    }
}

lazy_static! {
    static ref x_coords: Regex = Regex::new(r"x=(?P<x>\d+), y=(?P<y>\d+\.\.\d+|\d+)").unwrap();
    static ref y_coords: Regex = Regex::new(r"y=(?P<y>\d+), x=(?P<x>\d+\.\.\d+|\d+)").unwrap();
}

impl FromStr for Reservoir {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let (mut min_y, mut max_y) = (std::usize::MAX, 0);
        for line in s.trim().lines() {
            let line = line.trim();
            if x_coords.is_match(line) {
                let caps = x_coords.captures(line).unwrap();
                let x_val: usize = caps["x"].parse().unwrap();
                if caps["y"].contains("..") {
                    let mut y_range = caps["y"].split("..").map(|n| n.parse::<usize>().unwrap());
                    let y_start = y_range.next().unwrap();
                    let y_end = y_range.next().unwrap();
                    for n in y_start..=y_end {
                        min_y = min(min_y, n);
                        max_y = max(max_y, n);
                        tiles.insert((x_val, n).into(), State::Clay);
                    }
                } else {
                    let y_val: usize = caps["y"].parse().unwrap();
                    min_y = min(min_y, y_val);
                    max_y = max(max_y, y_val);
                    tiles.insert((x_val, y_val).into(), State::Clay);
                }
            } else if y_coords.is_match(line) {
                let caps = y_coords.captures(line).unwrap();
                let y_val: usize = caps["y"].parse().unwrap();
                min_y = min(min_y, y_val);
                max_y = max(max_y, y_val);
                if caps["x"].contains("..") {
                    let mut x_range = caps["x"].split("..").map(|n| n.parse::<usize>().unwrap());
                    let x_start = x_range.next().unwrap();
                    let x_end = x_range.next().unwrap();
                    for n in x_start..=x_end {
                        tiles.insert((n, y_val).into(), State::Clay);
                    }
                } else {
                    let x_val = caps["x"].parse().unwrap();
                    tiles.insert((x_val, y_val).into(), State::Clay);
                }
            }
        }

        Ok(Reservoir {
            tiles,
            min_y,
            max_y,
            visited: HashSet::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn input() {
        let count = solve_part1(include_str!("../input/tests/d17.txt"));
        assert_eq!(count, 57);
    }
}
