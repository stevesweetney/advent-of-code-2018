use min_max_heap::MinMaxHeap;
use regex::Regex;
use std::{
    char::ParseCharError,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

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

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut g = input
        .parse::<Graph>()
        .expect("Failed to parse graph from str");
    let mut processing = VecDeque::with_capacity(26);
    let mut total_time = 0;

    for (s, in_degree) in &g.in_degrees {
        if *in_degree == 0 {
            processing.push_back(*s);
        }
    }

    let mut q = TaskQueue::new_with_worker_num(5);

    while processing.len() > 0 || q.has_working() {
        let finished = q.process();
        total_time += 1;
        for c in finished {
            for adjacent in g.remove_node(c) {
                processing.push_back(*adjacent);
            }
        }

        while q.has_idle() && processing.len() > 0 {
            let step = processing.pop_front();
            q.add_task(step.unwrap());
        }
    }

    total_time - 1
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

struct TaskQueue {
    workers: Vec<Worker>,
}

impl TaskQueue {
    fn new_with_worker_num(num: u32) -> Self {
        let mut workers = Vec::new();
        for _ in 0..num {
            workers.push(Worker::new());
        }

        TaskQueue { workers }
    }

    fn process(&mut self) -> Vec<char> {
        self.workers
            .iter_mut()
            .filter_map(|w| w.process())
            .collect()
    }

    fn has_idle(&self) -> bool {
        for w in &self.workers {
            match w.state {
                State::Idle => {
                    return true;
                }
                _ => (),
            }
        }

        false
    }

    fn has_working(&self) -> bool {
        for w in &self.workers {
            match w.state {
                State::Active { .. } => {
                    return true;
                }
                _ => (),
            }
        }

        false
    }

    fn add_task(&mut self, task: char) -> bool {
        let mut idle_worker = None;
        for w in self.workers.iter_mut() {
            match w.state {
                State::Idle => {
                    idle_worker = Some(w);
                    break;
                }
                _ => (),
            }
        }

        match idle_worker {
            None => false,
            Some(w) => {
                w.state = State::Active {
                    task,
                    time_left: task_time(task),
                };
                true
            }
        }
    }
}

fn task_time(task: char) -> u8 {
    task as u8 - b'A' + 61
}

enum State {
    Idle,
    Active { task: char, time_left: u8 },
}

pub struct Worker {
    state: State,
}

impl Worker {
    fn new() -> Self {
        Worker { state: State::Idle }
    }

    fn process(&mut self) -> Option<char> {
        let res = match self.state {
            State::Idle => None,
            State::Active {
                ref task,
                ref mut time_left,
            } => {
                *time_left -= 1;
                if *time_left > 0 {
                    None
                } else {
                    Some(*task)
                }
            }
        };

        if res.is_some() {
            self.state = State::Idle;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/tests/d7.txt");
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

    #[test]
    fn test_task_time() {
        assert_eq!(task_time('A'), 61);
        assert_eq!(task_time('Z'), 86);
    }
}
