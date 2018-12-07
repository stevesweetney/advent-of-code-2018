use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref regexp: Regex = Regex::new(
        r"(?x)
        (aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg
        |hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn
        |oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu
        |vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz)
    "
    )
    .unwrap();
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut resulting_polymer = input.to_owned();
    while regexp.is_match(&resulting_polymer) {
        resulting_polymer = regexp.replace_all(&resulting_polymer, "").into_owned();
    }
    resulting_polymer.trim().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/tests/d5.txt");
        let input2 = "aabAAB";
        let input3 = "aA";
        assert_eq!(solve_part1(input), 10);
        assert_eq!(solve_part1(input2), input2.len());
        assert_eq!(solve_part1(input3), 0);
    }
}
