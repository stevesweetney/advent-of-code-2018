#[derive(Debug)]
pub struct Claim {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|line| {
            let (_, last) = line.split_at(line.find('@').unwrap() + 1);
            let parts = last.split(": ").collect::<Vec<&str>>();
            let edges = parts[0]
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<u32>>();
            let dimensions = parts[1]
                .split("x")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            Claim {
                left: edges[0],
                top: edges[1],
                width: dimensions[0],
                height: dimensions[1],
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Claim]) -> u32 {
    let mut fabric = [[0u8; 1000]; 1000];
    for c in input.iter() {
        create_claim(c, &mut fabric);
    }
    let mut two_or_more_claims = 0;
    for row in fabric.iter() {
        for col in row.iter() {
            if *col == 2 {
                two_or_more_claims += 1;
            }
        }
    }

    two_or_more_claims
}

fn create_claim(claim: &Claim, fabric: &mut [[u8; 1000]; 1000]) {
    for row in fabric
        .iter_mut()
        .skip(claim.top as usize)
        .take(claim.height as usize)
    {
        for col in row
            .iter_mut()
            .skip(claim.left as usize)
            .take(claim.width as usize)
        {
            if *col == 0 {
                *col = 1;
            } else {
                *col = 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../input/tests/d3.txt");
        let claims = input_gen(input);
        let result = solve_part1(&claims);
        assert_eq!(claims.len(), 3);
        assert_eq!(result, 4);
    }
}
