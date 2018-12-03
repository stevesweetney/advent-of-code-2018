use std::{collections::HashMap, i32};

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut two_count = 0;
    let mut three_count = 0;
    for line in input.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            let mut entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
        let mut containsTwo = false;
        let mut containsThree = false;
        for (_, o) in map.into_iter() {
            if o == 2 {
                containsTwo = true;
            } else if o == 3 {
                containsThree = true;
            }
        }
        if containsTwo {
            two_count += 1;
        }

        if containsThree {
            three_count += 1;
        }
    }

    return two_count * three_count;
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    let mut candidate1 = "";
    let mut candidate2 = "";
    for l1 in input.lines() {
        for l2 in input.lines() {
            let mut diff_count = 0;
            for (c1, c2) in l1.chars().zip(l2.chars()) {
                if (c1 != c2) {
                    diff_count += 1;
                }
            }

            if diff_count == 1 {
                candidate1 = l1;
                candidate2 = l2;
            }
        }
    }
    let mut res = String::new();
    for (c1, c2) in candidate1.chars().zip(candidate2.chars()) {
        if c1 == c2 {
            res.push(c1);
        }
    }
    return res;
}
