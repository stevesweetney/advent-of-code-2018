#[aoc_generator(day8)]
fn input_gen(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(list: &[u32]) -> u32 {
    let mut sum = 0;
    metadata_sum(list, &mut sum, 0);
    sum
}

fn metadata_sum(data: &[u32], sum: &mut u32, root: usize) -> usize {
    let mut child_count = data[root];

    let mut curr = root + 2;
    //let mut metadata_start = root;
    while child_count > 0 {
        curr = metadata_sum(data, sum, curr);
        child_count -= 1;
    }

    let metadata_end = curr + (data[root + 1] as usize);
    for m in curr..metadata_end {
        *sum += data[m];
    }

    return metadata_end;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_gen("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve_part1(&input), 138);
    }
}
