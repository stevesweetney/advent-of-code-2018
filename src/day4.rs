use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

lazy_static! {
    static ref regexp: Regex = Regex::new(
        r"(?x)
            \[[0-9]{4}
            -(?P<month>[0-9]{2})
            -(?P<day>[0-9]{2})
            \s+
            (?P<hour>[0-9]{2}):
            (?P<minute>[0-9]{2})
            \]
            \s+
            (?P<action>.*)
        "
    )
    .unwrap();
    static ref action_re: Regex = Regex::new(r"(?P<id>[0-9]+)").unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub action: Action,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    ShiftStart(u32),
    Sleep,
    Wake,
}

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = input
        .lines()
        .map(|l| {
            let caps = regexp.captures(l).unwrap();
            let action = {
                if caps["action"].contains("falls") {
                    Action::Sleep
                } else if caps["action"].contains("wakes") {
                    Action::Wake
                } else {
                    let c = action_re.captures(&caps["action"]).unwrap();
                    Action::ShiftStart(c["id"].parse().unwrap())
                }
            };
            Record {
                month: caps["month"].parse().unwrap(),
                day: caps["day"].parse().unwrap(),
                hour: caps["hour"].parse().unwrap(),
                minute: caps["minute"].parse().unwrap(),
                action,
            }
        })
        .collect();
    records.sort_unstable();
    records
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Record]) -> u32 {
    let (mut guard_on_duty, mut sleep_start) = (0, 0);
    let mut sleep_times: HashMap<u32, Vec<Range<u32>>> = HashMap::new();
    for rec in input {
        match rec.action {
            Action::ShiftStart(id) => guard_on_duty = id,
            Action::Sleep => sleep_start = rec.minute,
            Action::Wake => {
                sleep_times
                    .entry(guard_on_duty)
                    .or_insert(Vec::new())
                    .push(sleep_start as u32..rec.minute as u32);
            }
        }
    }
    let id = find_most_sleep(&sleep_times);
    let min = most_minute_slept(id, &sleep_times);
    id * min
}

fn find_most_sleep(sleep_times: &HashMap<u32, Vec<Range<u32>>>) -> u32 {
    let (mut max_sleep, mut id) = (0, 0);

    for (guard_id, ranges) in sleep_times {
        let time = ranges
            .iter()
            .cloned()
            .fold(0, |acc, r| acc + r.end - r.start);

        if time > max_sleep {
            max_sleep = time;
            id = *guard_id;
        }
    }

    id
}

fn most_minute_slept(guard_id: u32, sleep_times: &HashMap<u32, Vec<Range<u32>>>) -> u32 {
    let (mut most_frequent_minute, mut max_minute) = (0, 0);
    let mut freq_map = HashMap::new();
    let ranges = sleep_times.get(&guard_id).unwrap();

    for r in ranges.clone() {
        for m in r {
            let mut e = *freq_map.entry(m).or_insert(0);
            e += 1;
            freq_map.insert(m, e);
            if e > max_minute {
                max_minute = e;
                most_frequent_minute = m;
            }
        }
    }

    most_frequent_minute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = input_gen(include_str!("../input/tests/d4.txt"));
        assert_eq!(solve_part1(&input), 240);
    }
}
