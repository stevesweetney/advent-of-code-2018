use regex::Regex;
use std::{cmp, i32, num::ParseIntError, str::FromStr};

#[aoc_generator(day10)]
fn input_gen(input: &str) -> Vec<Light> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

// Part 2's answer is the number of seconds
#[aoc(day10, part1)]
fn solve_part1(input: &[Light]) -> u32 {
    let mut lights = input.to_vec();
    let mut rect = bounding_rect(&lights, Rectangle::new());
    let (mut width, mut height) = (rect.width(), rect.height());
    let mut seconds = 0;
    loop {
        for light in lights.iter_mut() {
            light.update()
        }

        rect = bounding_rect(&lights, rect);
        let w = rect.width();
        let h = rect.height();
        // The idea is that the points are slowly getting closer to each other
        if w > width && h > height {
            break;
        } else {
            width = w;
            height = h;
        }

        seconds += 1;
    }

    for light in lights.iter_mut() {
        light.undo();
    }

    draw(&lights, &rect);
    println!("area: {} seconds: {}", width * height, seconds);

    // Solver's need a return value for aoc-runner to work
    0
}

fn draw(lights: &[Light], rect: &Rectangle) {
    let mut lines = vec![vec!["."; rect.width() as usize]; rect.height() as usize];
    for mut p in lights.iter().cloned() {
        p.offset(rect.x1, rect.y1);
        lines[p.y as usize][p.x as usize] = "#";
    }

    for line in lines.iter().map(|l| l.join(" ")) {
        println!("{}", line);
    }
}

fn bounding_rect(lights: &[Light], rect: Rectangle) -> Rectangle {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, 0, i32::MAX, 0);

    for l in lights {
        min_x = cmp::min(min_x, l.x);
        max_x = cmp::max(max_x, l.x);
        min_y = cmp::min(min_y, l.y);
        max_y = cmp::max(max_y, l.y);
    }

    rect.update(min_x, max_x, min_y, max_y)
}

struct Rectangle {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Rectangle {
    fn new() -> Self {
        Rectangle {
            x1: 0,
            x2: 0,
            y1: 0,
            y2: 0,
        }
    }

    fn update(mut self, x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        self.x1 = x1;
        self.x2 = x2;
        self.y1 = y1;
        self.y2 = y2;
        self
    }

    fn area(&self) -> i32 {
        self.width() * self.height()
    }

    fn width(&self) -> i32 {
        i32::abs(self.x1 - self.x2)
    }

    fn height(&self) -> i32 {
        i32::abs(self.y1 - self.y2)
    }
}

#[derive(Clone)]
struct Light {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Light {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref regexp: Regex = Regex::new(
                r"(?x)
                position=<\s*(?P<x>-?\d+),\s* (?P<y>-?\d+)>\s+ velocity=<\s*(?P<vx>-?\d+),\s* (?P<vy>-?\d+)>
                "
            )
            .unwrap();
        }
        let caps = regexp.captures(s).unwrap();
        Ok(Light::new(
            caps["x"].parse()?,
            caps["y"].parse()?,
            caps["vx"].parse()?,
            caps["vy"].parse()?,
        ))
    }
}

impl Light {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Light { x, y, vx, vy }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn undo(&mut self) {
        self.x -= self.vx;
        self.y -= self.vy;
    }

    fn offset(&mut self, x_off: i32, y_off: i32) {
        self.x -= x_off;
        self.y -= y_off;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../input/tests/d10.rs");
        let lights = input_gen(input);
        assert_eq!(lights.len(), 31);
    }
}
