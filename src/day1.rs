use std::{collections::HashSet, i32};

#[aoc_generator(day1, part2)]
pub fn input_gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut set = HashSet::new();
    let mut freq = 0;
    set.insert(freq);
    for change in input.iter().cycle() {
        freq += change;
        if set.contains(&freq) {
            break;
        }
        set.insert(freq);
    }
    return freq;
}

#[cfg(test)]
mod tests {
    use super::{input_gen, solve_part1, solve_part2};

    #[test]
    fn should_return_3() {
        let input = "0\n-2\n5";
        assert_eq!(3, solve_part1(input));
    }

    #[test]
    fn should_return_55() {
        let data = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        let input = data.join("\n");
        assert_eq!((10 * 11) / 2, solve_part1(&input));
    }

    // part 2 tests
    #[test]
    fn should_return_0() {
        let data = vec!["1", "-1"];
        let data = data.join("\n");
        let input = input_gen(&data);
        assert_eq!(0, solve_part2(&input));
    }

    #[test]
    fn should_return_10() {
        let input = vec![3, 3, 4, -2, -4];
        assert_eq!(10, solve_part2(&input));
    }

    #[test]
    fn should_return_14() {
        let input = vec![7, 7, -2, -7, -4];
        assert_eq!(14, solve_part2(&input));
    }
}
