use kdtree::KdTree;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    f32,
    num::ParseIntError,
    str::FromStr,
    u32::MAX,
};

#[cfg(not(test))]
const PART2_CUTOFF: f32 = 10000.0;

fn manhattan_distance(p1: &[f32], p2: &[f32]) -> f32 {
    p1.iter()
        .zip(p2)
        .fold(0.0, |acc, (a, b)| acc + f32::abs(a - b))
}

#[aoc_generator(day6)]
fn input_gen(input: &str) -> Vec<Point> {
    input.lines().map(|l| l.parse::<Point>().unwrap()).collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Point]) -> u32 {
    let (rect, points_tree) = process_tree_and_rect(input);
    let mut infinite_areas = HashSet::<usize>::new();

    // My thought process here is that
    // the points that are nearest to coordinates on the perimeter
    // of the bounding rectangle probably span infinitely
    for p in rect.perimeter_list() {
        let nearest = points_tree
            .nearest(&[p.x as f32, p.y as f32], 2, &manhattan_distance)
            .unwrap();
        if nearest[0].0 != nearest[1].0 {
            infinite_areas.insert(*nearest[0].1);
        }
    }

    let mut area_map = HashMap::new();
    let mut best = 0;
    for p in rect.points_iter() {
        let nearest = points_tree
            .nearest(&[p.0 as f32, p.1 as f32], 2, &manhattan_distance)
            .unwrap();
        // Check that the nearest 2 points are not tied and that
        // the nearest point has a finite area
        if nearest[0].0 != nearest[1].0 && !infinite_areas.contains(nearest[0].1) {
            let area = area_map.entry(nearest[0].1).or_insert(0);
            *area += 1;
            best = cmp::max(best, *area);
        }
    }
    best
}

#[aoc(day6, part2)]
fn solve_part2(input: &[Point]) -> usize {
    let (rect, points_tree) = process_tree_and_rect(input);
    let mut points_in_region = HashSet::new();

    // Just go through all the coordinates in the bounding rectangle
    // marking those that have a total distance to all points are less
    // than the cutoff
    for (i, p) in rect.points_iter().enumerate() {
        let total_distance = points_tree
            .iter_nearest(&[p.0 as f32, p.1 as f32], &manhattan_distance)
            .unwrap()
            .fold(0.0, |acc, (distance, _)| distance + acc);
        if total_distance < PART2_CUTOFF {
            points_in_region.insert(i);
        }
    }

    points_in_region.len()
}

fn process_tree_and_rect(points: &[Point]) -> (Rectangle, PointsTree) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (MAX, 0, MAX, 0);
    let mut tree = KdTree::new_with_capacity(2, points.len());

    // The idea here is to create the smallest bounding rectangle that contains all
    // of the point
    for (i, p) in points.iter().enumerate() {
        min_x = cmp::min(min_x, p.x);
        min_y = cmp::min(min_y, p.y);
        max_x = cmp::max(max_x, p.x);
        max_y = cmp::max(max_y, p.y);
        tree.add([p.x as f32, p.y as f32], i);
    }

    (
        Rectangle {
            upper_left: Point::new(min_x, min_y),
            upper_right: Point::new(max_y, min_y),
            lower_left: Point::new(min_x, max_y),
            lower_right: Point::new(max_x, max_y),
        },
        tree,
    )
}

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(",").map(|s| s.trim()).collect();
        Ok(Point {
            x: coords[0].parse::<u32>()?,
            y: coords[1].parse::<u32>()?,
        })
    }
}

type PointsTree = KdTree<f32, usize, [f32; 2]>;

struct Rectangle {
    upper_left: Point,
    upper_right: Point,
    lower_left: Point,
    lower_right: Point,
}

impl Rectangle {
    fn perimeter_list(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in self.upper_left.x..=self.upper_right.x {
            points.push(Point::new(x, self.upper_left.y));
            points.push(Point::new(x, self.lower_left.y));
        }

        for y in self.upper_left.y..=self.lower_left.y {
            points.push(Point::new(self.upper_left.x, y));
            points.push(Point::new(self.upper_right.x, y));
        }

        points
    }

    fn points_iter(&self) -> PointsIter {
        PointsIter {
            rect: self,
            px: self.upper_left.x,
            py: self.upper_left.y,
        }
    }
}

struct PointsIter<'r> {
    rect: &'r Rectangle,
    px: u32,
    py: u32,
}

impl<'r> Iterator for PointsIter<'r> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<(u32, u32)> {
        if self.px > self.rect.upper_right.x {
            self.px = self.rect.upper_left.x;
            self.py += 1;
        }
        if self.py > self.rect.lower_right.y {
            return None;
        }
        let (x, y) = (self.px, self.py);
        self.px += 1;
        Some((x, y))
    }
}

#[cfg(test)]
const PART2_CUTOFF: f32 = 32.0;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_gen(include_str!("../input/tests/d6.txt"));
        assert_eq!(solve_part1(&input), 17);
    }

    #[test]
    fn test_part2() {
        let input = input_gen(include_str!("../input/tests/d6.txt"));
        assert_eq!(solve_part2(&input), 16);
    }

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(manhattan_distance(&[0.0, 0.0], &[3.0, 10.0]), 13.0);
    }
}
