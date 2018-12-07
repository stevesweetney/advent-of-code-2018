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
    react_polymer(input.to_owned())
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let lowercase_alpha = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_ascii_lowercase())
        .collect::<Vec<char>>();
    let mut min_length = input.len();
    for c in lowercase_alpha {
        let resulting_polymer = remove_one_type(input, c);
        let curr_length = react_polymer(resulting_polymer);
        if min_length > curr_length {
            min_length = curr_length;
        }
    }

    min_length
}

pub fn react_polymer(mut polymer: String) -> usize {
    while regexp.is_match(&polymer) {
        polymer = regexp.replace_all(&polymer, "").into_owned();
    }
    polymer.trim().len()
}

pub fn remove_one_type(polymer: &str, polymer_type: char) -> String {
    let uppercase = polymer_type.to_uppercase().to_string();
    let type_regexp = Regex::new(&format!("({}|{})", polymer_type, uppercase,)).unwrap();
    type_regexp.replace_all(polymer, "").into_owned()
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

    #[test]
    fn test_part2() {
        let input = include_str!("../input/tests/d5.txt");
        assert_eq!(solve_part2(input), 4);
    }
}
