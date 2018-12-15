use std::collections::HashMap;

type Track = Vec<Vec<char>>;

// Prints (x, y) coordinate of collision instead of returning
#[aoc(day13, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut tracks: Track = {
        input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect()
    };
    let mut carts: Vec<Cart> = init_carts(&mut tracks);
    let mut collision_at = (0, 0);
    let mut ticks = 0;
    let is_cart = |c: char| match c {
        '^' | '<' | '>' | 'v' => true,
        _ => false,
    };

    let check_intersection_or_collision = |cart: &mut Cart| match cart.track {
        '+' => {
            match cart.intersection {
                0 => cart.turn_left(),
                2 => cart.turn_right(),
                _ => (),
            }
            cart.intersection += 1;
            cart.intersection %= 3;
            false
        }
        '^' | '<' | '>' | 'v' => {
            println!("Collision at ({}, {})", cart.col, cart.row);
            true
        }

        _ => false,
    };
    loop {
        for y in 0..tracks.len() {
            for x in 0..tracks[y].len() {
                if is_cart(tracks[y][x]) {
                    if let Some(c) = carts
                        .iter_mut()
                        .find(|c| c.col == x && c.row == y && !c.moved)
                    {
                        let mut found_collision = false;
                        c.moved = true;

                        tracks[y][x] = c.track;
                        match c.direction {
                            Direction::Up => {
                                c.row -= 1;
                                c.track = tracks[y - 1][x];
                                match c.track {
                                    '/' => c.direction = Direction::Right,
                                    '\\' => c.direction = Direction::Left,
                                    _ => found_collision = check_intersection_or_collision(c),
                                }
                            }
                            Direction::Left => {
                                c.col -= 1;
                                c.track = tracks[y][x - 1];
                                match c.track {
                                    '/' => c.direction = Direction::Down,
                                    '\\' => c.direction = Direction::Up,
                                    _ => found_collision = check_intersection_or_collision(c),
                                }
                            }
                            Direction::Down => {
                                c.row += 1;
                                c.track = tracks[y + 1][x];
                                match c.track {
                                    '/' => c.direction = Direction::Left,
                                    '\\' => c.direction = Direction::Right,
                                    _ => found_collision = check_intersection_or_collision(c),
                                }
                            }
                            Direction::Right => {
                                c.col += 1;
                                c.track = tracks[y][x + 1];
                                match c.track {
                                    '/' => c.direction = Direction::Up,
                                    '\\' => c.direction = Direction::Down,
                                    _ => found_collision = check_intersection_or_collision(c),
                                }
                            }
                            _ => (),
                        }

                        if found_collision {
                            collision_at = (c.col, c.row);
                            println!("TICKS: {}", ticks + 1);
                            return 0;
                        }

                        tracks[c.row][c.col] = match c.direction {
                            Direction::Up => '^',
                            Direction::Left => '<',
                            Direction::Down => 'v',
                            Direction::Right => '>',
                        };
                    }
                }
            }
        }
        for cart in carts.iter_mut() {
            cart.moved = false;
        }
        ticks += 1;
    }

    0
}

fn init_carts(tracks: &mut Track) -> Vec<Cart> {
    let mut carts: Vec<Cart> = vec![];
    for (y, row) in tracks.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            match *col {
                '^' => carts.push(Cart::new(Direction::Up, y, x)),
                '<' => carts.push(Cart::new(Direction::Left, y, x)),
                '>' => carts.push(Cart::new(Direction::Right, y, x)),
                'v' => carts.push(Cart::new(Direction::Down, y, x)),
                _ => (),
            }
        }
    }

    carts
}

struct Cart {
    direction: Direction,
    track: char, // type of track the cart is on
    // Determine which way to go at an intersection
    // 0 - left, 1 - straight, 2 - right
    intersection: u8,
    row: usize,
    col: usize,
    moved: bool,
}

impl Cart {
    // Assuming that none of the carts begin on curves
    fn new(direction: Direction, row: usize, col: usize) -> Self {
        let track = match direction {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        };
        Cart {
            direction,
            track,
            intersection: 0,
            row,
            col,
            moved: false,
        }
    }

    fn turn_left(&mut self) {
        use self::Direction::*;
        let d = match self.direction {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        };

        self.direction = d;
    }

    fn turn_right(&mut self) {
        use self::Direction::*;
        let d = match self.direction {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        };

        self.direction = d;
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/tests/d13.txt");

        assert_eq!(solve_part1(input), (7, 3));
    }

    #[test]
    fn test_turns() {
        let mut c1 = Cart::new(Direction::Up, 0, 0);
        let mut c2 = Cart::new(Direction::Right, 1, 1);

        c1.turn_right();
        c2.turn_left();
        assert_eq!(c1.direction, Direction::Right);
        assert_eq!(c2.direction, Direction::Up);

        c1.turn_right();
        c1.turn_right();
        c1.turn_right();
        assert_eq!(c1.direction, Direction::Up);
    }
}
