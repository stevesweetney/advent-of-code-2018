use std::fmt::{self, Display};

const GRID_SERIAL_NUMBER: usize = 1133;

#[aoc(day11, part1)]
fn solve_part1(_: &str) -> Coords {
    let mut grid = vec![vec![0; 300]; 300];
    populate_fuel_cells(&mut grid);
    let (x, y, _, _) = square_total(&grid, 3);

    Coords {
        x: x as u32,
        y: y as u32,
        size: 3,
    }
}

fn square_total(grid: &Vec<Vec<i32>>, square_size: usize) -> (usize, usize, usize, i32) {
    let mut best = 0;
    let mut coords = (1, 1);

    for y in 0..300 - square_size {
        for x in 0..300 - square_size {
            let mut square_total = 0;
            for s_y in y..y + square_size {
                for s_x in x..x + square_size {
                    square_total += grid[s_y][s_x];
                }
            }

            if square_total > best {
                best = square_total;
                coords = (x + 1, y + 1);
            }
        }
    }

    (coords.0, coords.1, square_size, best)
}

fn populate_fuel_cells(grid: &mut Vec<Vec<i32>>) {
    for (y, row) in grid.iter_mut().enumerate() {
        let y_offset = y + 1;

        for (x, cell) in row.iter_mut().enumerate() {
            let x_offset = x + 1;

            
            let rack_id = x_offset + 10;
            let mut fuel = rack_id * y_offset;
            fuel += GRID_SERIAL_NUMBER;
            fuel *= rack_id;

            if fuel < 100 {
                *cell = 0;
            } else {
                fuel /= 100;
                fuel %= 10;
                fuel -= 4;
                *cell = fuel as i32;
            }
        }
    }
}

#[derive(Debug)]
struct Coords {
    x: u32,
    y: u32,
    size: u32,
}

impl Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) Square Size: {}", self.x, self.y, self.size)
    }
}
