use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

// #[aoc_generator(day16)]
// fn input(input: &str) -> Vec<&str> {

// }

fn input_gen(input: &str) -> (u32, HashMap<u32, String>, &str) {
    let mut parts = input.split("\n\n\n");
    let part1 = parts.next().unwrap();
    let (part1_answer, op_map) = run_samples(
        part1
            .split("\n\n")
            .map(|s| s.trim().parse::<Sample>().unwrap())
            .collect(),
    );
    (part1_answer, op_map, parts.next().unwrap())
}

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> u32 {
    input_gen(input).0
}

#[aoc(day16, part2)]
fn solve_part2(input: &str) -> usize {
    let (_, op_map, part2_input) = input_gen(input);
    let mut model = Model::new(op_map);
    for line in part2_input.trim().lines() {
        let mut instruction: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        model.execute(&instruction);
    }
    println!("registers: {:?}", &model.registers);
    model.registers[0]
}

fn run_samples(samples: Vec<Sample>) -> (u32, HashMap<u32, String>) {
    let mut three_or_more = 0;
    let mut op_map: HashMap<u32, HashSet<String>> = HashMap::new();
    for s in &samples {
        let mut possible_op = 0;
        let mut result = s.after[s.output];
        let mut opcode_candidates = HashSet::new();
        // check addr
        if s.before[s.input_a] + s.before[s.input_b] == result {
            opcode_candidates.insert("addr".to_owned());
            possible_op += 1;
        }
        // check addi
        if s.before[s.input_a] + s.input_b == result {
            opcode_candidates.insert("addi".to_owned());
            possible_op += 1;
        }
        // check mulr
        if s.before[s.input_a] * s.before[s.input_b] == result {
            opcode_candidates.insert("mulr".to_owned());
            possible_op += 1;
        }
        // check muli
        if s.before[s.input_a] * s.input_b == result {
            opcode_candidates.insert("muli".to_owned());
            possible_op += 1;
        }
        // check banr
        if s.before[s.input_a] & s.before[s.input_b] == result {
            opcode_candidates.insert("banr".to_owned());
            possible_op += 1;
        }
        // check bani
        if s.before[s.input_a] & s.input_b == result {
            opcode_candidates.insert("bani".to_owned());
            possible_op += 1;
        }
        // check borr
        if s.before[s.input_a] | s.before[s.input_b] == result {
            opcode_candidates.insert("borr".to_owned());
            possible_op += 1;
        }
        // check bori
        if s.before[s.input_a] | s.input_b == result {
            opcode_candidates.insert("bori".to_owned());
            possible_op += 1;
        }
        // check setr
        if s.before[s.input_a] == result {
            opcode_candidates.insert("setr".to_owned());
            possible_op += 1;
        }
        // check seti
        if s.input_a == result {
            opcode_candidates.insert("seti".to_owned());
            possible_op += 1;
        }

        if result == 0 || result == 1 {
            let bool_result = match result {
                0 => false,
                1 => true,
                _ => unreachable!(),
            };
            // check gtir
            if (s.input_a > s.before[s.input_b]) == bool_result {
                opcode_candidates.insert("gtir".to_owned());
                possible_op += 1;
            }
            // check gtri
            if (s.before[s.input_a] > s.input_b) == bool_result {
                opcode_candidates.insert("gtri".to_owned());
                possible_op += 1;
            }
            // check gtrr
            if (s.before[s.input_a] > s.before[s.input_b]) == bool_result {
                opcode_candidates.insert("gtrr".to_owned());
                possible_op += 1;
            }

            // check eqir
            if (s.input_a == s.before[s.input_b]) == bool_result {
                opcode_candidates.insert("eqir".to_owned());
                possible_op += 1;
            }
            // check eqri
            if (s.before[s.input_a] == s.input_b) == bool_result {
                opcode_candidates.insert("eqri".to_owned());
                possible_op += 1;
            }
            // check eqrr
            if (s.before[s.input_a] == s.before[s.input_b]) == bool_result {
                opcode_candidates.insert("eqrr".to_owned());
                possible_op += 1;
            }
        }

        if possible_op >= 3 {
            three_or_more += 1;
        }

        if let Some(opcodes) = op_map.get(&s.opcode) {
            op_map.insert(
                s.opcode,
                opcodes.intersection(&opcode_candidates).cloned().collect(),
            );
        } else {
            op_map.insert(s.opcode, opcode_candidates);
        }
    }

    (three_or_more, final_opcode_map(op_map))
}

fn final_opcode_map(mut m: HashMap<u32, HashSet<String>>) -> HashMap<u32, String> {
    let mut result = HashMap::new();
    while result.len() < 16 {
        let mut removed = Vec::new();
        for (opcode, names) in &m {
            if names.len() == 1 {
                let name = names.iter().next().unwrap().clone();
                result.insert(*opcode, name.clone());
                removed.push(name);
            }
        }

        for (opcode, names) in &mut m {
            for r in &removed {
                names.remove(r);
            }
        }
    }

    result
}

struct Model {
    op_map: HashMap<u32, String>,
    registers: [usize; 4],
}

impl Model {
    fn new(op_map: HashMap<u32, String>) -> Self {
        Model {
            op_map,
            registers: [0, 0, 0, 0],
        }
    }

    fn execute(&mut self, instruction: &[usize]) {
        let (opcode, a, b, c) = (
            instruction[0] as u32,
            instruction[1],
            instruction[2],
            instruction[3],
        );
        match self.op_map[&opcode].as_ref() {
            "addr" => {
                self.registers[c] = self.registers[a] + self.registers[b];
            }
            "addi" => {
                self.registers[c] = self.registers[a] + b;
            }
            "mulr" => {
                self.registers[c] = self.registers[a] * self.registers[b];
            }
            "muli" => {
                self.registers[c] = self.registers[a] * b;
            }
            "banr" => {
                self.registers[c] = self.registers[a] & self.registers[b];
            }
            "bani" => {
                self.registers[c] = self.registers[a] & b;
            }
            "borr" => {
                self.registers[c] = self.registers[a] | self.registers[b];
            }
            "bori" => {
                self.registers[c] = self.registers[a] | b;
            }
            "setr" => {
                self.registers[c] = self.registers[a];
            }
            "seti" => {
                self.registers[c] = a;
            }
            "gtir" => {
                self.registers[c] = (a > self.registers[b]) as usize;
            }
            "gtri" => {
                self.registers[c] = (self.registers[a] > b) as usize;
            }
            "gtrr" => {
                self.registers[c] = (self.registers[a] > self.registers[b]) as usize;
            }
            "eqir" => {
                self.registers[c] = (a == self.registers[b]) as usize;
            }
            "eqri" => {
                self.registers[c] = (self.registers[a] == b) as usize;
            }
            "eqrr" => {
                self.registers[c] = (self.registers[a] == self.registers[b]) as usize;
            }
            _ => panic!("unexpected opcode"),
        }
    }
}

#[derive(Debug)]
struct Sample {
    before: [usize; 4],
    after: [usize; 4],
    input_a: usize,
    input_b: usize,
    output: usize,
    opcode: u32,
}

impl FromStr for Sample {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref regexep: Regex = Regex::new(
                r"(?x)
                (?-x)Before: \[(?P<b0>\d), (?P<b1>\d), (?P<b2>\d), (?P<b3>\d)\](?x)\n
                (?-x)(?P<OP>\d+) (?P<A>\d) (?P<B>\d) (?P<C>\d)(?x)\n
                (?-x)After:  \[(?P<a0>\d), (?P<a1>\d), (?P<a2>\d), (?P<a3>\d)\]"
            )
            .unwrap();
        }
        assert!(regexep.is_match(s), "regex does not match string");
        let caps = regexep.captures(s).unwrap();
        let before = [
            caps["b0"].parse()?,
            caps["b1"].parse()?,
            caps["b2"].parse()?,
            caps["b3"].parse()?,
        ];

        let after = [
            caps["a0"].parse()?,
            caps["a1"].parse()?,
            caps["a2"].parse()?,
            caps["a3"].parse()?,
        ];

        Ok(Sample {
            before,
            after,
            opcode: caps["OP"].parse()?,
            input_a: caps["A"].parse()?,
            input_b: caps["B"].parse()?,
            output: caps["C"].parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../input/tests/d16.txt");
        input_gen_part1(input);
        assert!(true);
    }
}
