const NUM_RECIPES: usize = 430971;

#[aoc(day14, part1)]
fn solve_part1(_: &str) -> u32 {
    let mut recipes = vec![3, 7];
    let (mut elf_1, mut elf_2) = (0, 1);
    loop {
        let mut sum = recipes[elf_1] + recipes[elf_2];
        //println!("{:?}, elf_1: {} elf_2: {}", recipes, elf_1, elf_2);
        if sum < 10 {
            recipes.push(sum);
        } else {
            let second = sum % 10;
            sum /= 10;
            let first = sum % 10;
            recipes.push(first);
            recipes.push(second);
        }

        elf_1 += (1 + recipes[elf_1]);
        elf_1 %= recipes.len();
        elf_2 += (1 + recipes[elf_2]);
        elf_2 %= recipes.len();
        if recipes.len() > NUM_RECIPES + 10 {
            break;
        }
    }
    for r in recipes.iter().skip(NUM_RECIPES).take(10) {
        print!("{}", r);
    }
    println!("");
    0
}

#[aoc(day14, part2)]
fn solve_part2(_: &str) -> u32 {
    let mut recipes = vec![3, 7];
    let (mut elf_1, mut elf_2) = (0, 1);
    let mut curr_digits = vec![];
    let pattern = to_digits(NUM_RECIPES);
    let mut p_i = 0;
    let mut r_i = 0;
    let mut i = 0;
    loop {
        let mut sum = recipes[elf_1] + recipes[elf_2];
        //println!("{:?}, elf_1: {} elf_2: {}", recipes, elf_1, elf_2);
        curr_digits.push(recipes[i]);
        if sum < 10 {
            recipes.push(sum);
        } else {
            let second = sum % 10;
            sum /= 10;
            let first = sum % 10;
            recipes.push(first);
            recipes.push(second);
        }

        elf_1 += (1 + recipes[elf_1]);
        elf_1 %= recipes.len();
        elf_2 += (1 + recipes[elf_2]);
        elf_2 %= recipes.len();

        if curr_digits[r_i] == pattern[p_i] {
            if curr_digits.len() == pattern.len() {
                i -= pattern.len() - 1;
                break;
            }
            r_i += 1;
            p_i += 1;
        } else {
            curr_digits.clear();
            p_i = 0;
            r_i = 0;
            if recipes[i] == pattern[p_i] {
                curr_digits.push(recipes[i]);
                r_i += 1;
                p_i += 1;
            }
        }
        i += 1;
    }
    //println!("{:?}", recipes);
    i as u32
}

fn to_digits(n: usize) -> Vec<usize> {
    n.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect()
}
