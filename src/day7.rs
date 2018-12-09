use min_max_heap::MinMaxHeap;
use regex::Regex;
use std::{char::ParseCharError, collections::HashMap, str::FromStr};

lazy_static! {
    static ref regexp: Regex = Regex::new(
        r"(?x)
        Step \s+ (?P<from>[A-Z]{1})
        (?-x) must be finished before step (?x) \s*
        (?P<to>[A-Z]{1})
    "
    )
    .unwrap();
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> String {
    let mut g = input
        .parse::<Graph>()
        .expect("Failed to parse graph from str");
    let mut processing = MinMaxHeap::with_capacity(26);
    let mut steps = String::new();

    // Topological sort!
    for (s, in_degree) in &g.in_degrees {
        if *in_degree == 0 {
            processing.push(*s);
        }
    }

    while let Some(step) = processing.pop_min() {
        steps.push(step);
        for adjacent in g.remove_node(step) {
            processing.push(*adjacent);
        }
    }

    steps
}

struct Graph {
    adjacency_list: HashMap<char, Vec<char>>,
    in_degrees: HashMap<char, u32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            in_degrees: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: char, to: char) {
        self.adjacency_list
            .entry(from)
            .or_insert(Vec::new())
            .push(to);
        *self.in_degrees.entry(to).or_insert(0) += 1;
        self.in_degrees.entry(from).or_insert(0);
        self.adjacency_list.entry(to).or_insert(Vec::new());
    }

    fn remove_node(&mut self, node: char) -> Vec<&char> {
        let mut ready_to_process = Vec::new();
        for adjacent in self.adjacency_list.get(&node).unwrap() {
            let in_degree = self.in_degrees.entry(*adjacent).or_insert(1);
            *in_degree -= 1;
            if *in_degree == 0 {
                ready_to_process.push(adjacent);
            }
        }

        ready_to_process
    }

    fn len(&self) -> usize {
        self.adjacency_list.len()
    }
}

impl FromStr for Graph {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = Graph::new();
        for line in s.lines() {
            let caps = regexp.captures(line).expect("No captures found");
            g.add_edge(caps["from"].parse().unwrap(), caps["to"].parse().unwrap());
        }

        Ok(g)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/tests/d7.txt");
        let g: Graph = input.parse().unwrap();
        assert_eq!(&solve_part1(&input), "CABDFE");
    }

    #[test]
    fn test_regex() {
        let line = "Step G must be finished before step T can begin.";
        let caps = regexp.captures(line).expect("Found no captures");
        let (from, to) = ("from".to_owned(), "to".to_owned());
        let formatted = format!(
            "FROM: {} TO: {}",
            caps["from"].parse::<char>().unwrap(),
            caps["to"].parse::<char>().unwrap()
        );
        assert_eq!(formatted, "FROM: G TO: T");
    }
}
