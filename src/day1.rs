use std::i32;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::solve_part1;

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
}
