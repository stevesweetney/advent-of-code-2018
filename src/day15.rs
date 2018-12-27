use slotmap::{DefaultKey, SecondaryMap, SlotMap};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::{char::ParseCharError, convert::From, str::FromStr};

const HIT_POINTS: i32 = 200;
const ATTACK_POWER: i32 = 3;

fn input_gen(input: &str) -> Cave {
    input.parse().unwrap()
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut cave = input_gen(input);
    let mut round = 0;
    while !cave.step() {
        round += 1;
    }

    round * cave.sum_units_hp()
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut elf_attack = 4;
    let mut outcome = 0;
    loop {
        let mut cave = input_gen(input);
        let mut round = 0;
        let start_count = cave.elf_count();
        cave.elf_attack = elf_attack;
        while !cave.step() {
            round += 1
        }
        if start_count == cave.elf_count() {
            outcome = cave.sum_units_hp() * round;
            break;
        }

        elf_attack += 1;
    }
    outcome
}

struct Cave {
    elf_attack: i32,
    adjacency_list: BTreeMap<Coord, Vec<Coord>>,
    occupied: BTreeMap<Coord, DefaultKey>,
    units: SlotMap<DefaultKey, Unit>,
    unit_tiles: SecondaryMap<DefaultKey, Coord>,
}

impl Cave {
    fn step(&mut self) -> bool {
        use self::Side::*;
        let unit_coords: Vec<(Coord, DefaultKey)> =
            self.occupied.iter().map(|(&k, &v)| (k, v)).collect();
        for (mut tile, u) in unit_coords.into_iter() {
            if let None = self.units.get(u) {
                continue;
            }
            let mut unit = self.units[u];
            if unit.dead {
                continue;
            }
            let mut targets = self.find_targets(&unit);

            if targets.is_empty() {
                return true;
            }

            self.move_unit(&u, &tile, targets.as_slice());
            self.unit_attack(&u, targets.as_slice());
        }

        false
    }

    fn move_unit(&mut self, unit_id: &DefaultKey, unit_pos: &Coord, targets: &[Coord]) {
        if let Some(t_pos) = self.move_to(unit_pos, targets) {
            self.unit_tiles[*unit_id] = t_pos;
            self.occupied.insert(t_pos, unit_id.to_owned());
            self.occupied.remove(unit_pos);
        }
    }

    fn unit_attack(&mut self, unit_id: &DefaultKey, targets: &[Coord]) {
        if let Some(enemy_pos) = self.can_attack_targets(unit_id, targets) {
            if let Some(enemy_id) = self.occupied.get(&enemy_pos) {
                let attack_power = match self.units[*unit_id].side {
                    Side::Goblin => ATTACK_POWER,
                    Side::Elf => self.elf_attack,
                };
                let mut enemy_unit = self.units.get_mut(*enemy_id).unwrap();
                enemy_unit.hit_points -= attack_power;

                if enemy_unit.hit_points <= 0 {
                    enemy_unit.dead = true;
                    self.units.remove(*enemy_id);
                    self.occupied.remove(&enemy_pos);
                }
            }
        }
    }

    fn find_targets(&self, unit: &Unit) -> Vec<Coord> {
        let mut targets = Vec::new();
        for (id, other) in &self.units {
            if unit.oppose(other) {
                targets.push(self.unit_tiles[id])
            }
        }
        targets
    }

    fn can_attack_targets(&self, unit_id: &DefaultKey, targets_pos: &[Coord]) -> Option<Coord> {
        let unit = self.units[*unit_id];
        let unit_pos = self.unit_tiles[*unit_id];
        let mut enemy_pos: Option<Coord> = None;
        let mut min_health = HIT_POINTS;
        for pos in targets_pos {
            if self.adjacency_list[&unit_pos].contains(pos) {
                if let Some(e_pos) = enemy_pos {
                    let enemy_unit = self.units[self.occupied[pos]];
                    match (enemy_unit.hit_points, *pos).cmp(&(min_health, e_pos)) {
                        Ordering::Less => {
                            min_health = enemy_unit.hit_points;
                            enemy_pos = Some(*pos);
                        }
                        _ => (),
                    }
                } else {
                    enemy_pos = Some(*pos);
                    let enemy_unit = self.units[self.occupied[pos]];
                    min_health = enemy_unit.hit_points;
                }
            }
        }

        enemy_pos
    }

    fn move_to(&self, unit_pos: &Coord, targets_pos: &[Coord]) -> Option<Coord> {
        if !self.in_range(unit_pos, targets_pos) {
            let mut paths = self.paths_to_targets(unit_pos.to_owned(), targets_pos);
            if paths.is_empty() {
                return None;
            }
            let mut sorted: Vec<_> = paths
                .iter()
                .map(|path| (path.last().unwrap().clone(), path[1].clone()))
                .collect();
            sorted.sort_unstable();
            return Some(sorted[0].1);
        }

        None
    }

    fn in_range(&self, unit_pos: &Coord, targets_pos: &[Coord]) -> bool {
        for pos in targets_pos {
            if self.adjacency_list[unit_pos].contains(pos) {
                return true;
            }
        }

        false
    }

    fn paths_to_targets<'t>(&self, tile_pos: Coord, targets_pos: &'t [Coord]) -> Vec<Vec<Coord>> {
        let mut seen: HashSet<_> = self
            .occupied
            .keys()
            .filter(|c| **c != tile_pos)
            .cloned()
            .collect();
        let mut min_len_to_target = None;
        let mut results = Vec::new();
        let mut q = VecDeque::new();
        q.push_front((0, vec![tile_pos.clone()]));
        while !q.is_empty() {
            let (path_len, mut path) = q.pop_back().unwrap();
            let tile = path.last().unwrap();

            if let Some(best) = min_len_to_target {
                if path_len > best {
                    return results;
                }
            }

            if self.adjacency_list[&tile]
                .iter()
                .any(|a| targets_pos.contains(a))
            {
                let mut len = path_len + 1;

                let curr_target_tile = self.adjacency_list[&tile]
                    .iter()
                    .cloned()
                    .find(|a| targets_pos.contains(a))
                    .unwrap();
                path.push(curr_target_tile.clone());
                
                if let Some(best) = min_len_to_target {
                    if len > best {
                        return results;
                    }
                } else {
                    min_len_to_target = Some(len);
                    results.push(path);
                }
                continue;
            }

            if seen.contains(&tile) {
                continue;
            }
            seen.insert(*tile);
            for adjacent in &self.adjacency_list[&tile] {
                if !seen.contains(adjacent) {
                    let mut p = path.clone();
                    p.push(adjacent.to_owned());
                    q.push_front((path_len + 1, p));
                }
            }
        }

        results
    }

    fn sum_units_hp(&self) -> i32 {
        self.units.values().map(|u| u.hit_points).sum()
    }

    fn elf_count(&self) -> usize {
        let mut count = 0;
        for unit in self.units.values() {
            match unit.side {
                Side::Elf => count += 1,
                Side::Goblin => (),
            }
        }

        count
    }
}

impl FromStr for Cave {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adjacency_list: BTreeMap<_, Vec<_>> = BTreeMap::new();
        let grid: Vec<Vec<_>> = s.lines().map(|l| l.trim().chars().collect()).collect();
        let mut occupied = BTreeMap::new();
        let mut units = SlotMap::new();
        let mut unit_tiles = SecondaryMap::new();
        let n = grid.len();
        for y in 0..n {
            let m = grid[y].len();
            for x in 0..m {
                if grid[y][x] != '#' {
                    match grid[y][x] {
                        'G' => {
                            let k = units.insert(Unit::new(Side::Goblin));
                            unit_tiles.insert(k, (x, y).into());
                            occupied.insert((x, y).into(), k);
                        }
                        'E' => {
                            let k = units.insert(Unit::new(Side::Elf));
                            unit_tiles.insert(k, (x, y).into());
                            occupied.insert((x, y).into(), k);
                        }
                        '.' => (),
                        _ => panic!("Unexpected char"),
                    }
                    if x + 1 < m && grid[y][x + 1] != '#' {
                        adjacency_list
                            .entry((x, y).into())
                            .or_insert(Vec::new())
                            .push((x + 1, y).into());
                        adjacency_list
                            .entry((x + 1, y).into())
                            .or_insert(Vec::new())
                            .push((x, y).into());
                    }

                    if y + 1 < n && grid[y + 1][x] != '#' {
                        adjacency_list
                            .entry((x, y).into())
                            .or_insert(Vec::new())
                            .push((x, y + 1).into());
                        adjacency_list
                            .entry((x, y + 1).into())
                            .or_insert(Vec::new())
                            .push((x, y).into());
                    }
                }
            }
        }

        Ok(Cave {
            elf_attack: ATTACK_POWER,
            adjacency_list,
            occupied,
            units,
            unit_tiles,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.y, self.x).partial_cmp(&(other.y, other.x))
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<(usize, usize)> for Coord {
    fn from(t: (usize, usize)) -> Self {
        Coord { x: t.0, y: t.1 }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Side {
    Elf,
    Goblin,
}

impl Unit {
    fn oppose(&self, other: &Unit) -> bool {
        match self.side {
            Side::Goblin => match other.side {
                Side::Elf => true,
                _ => false,
            },
            Side::Elf => match other.side {
                Side::Goblin => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Unit {
    side: Side,
    hit_points: i32,
    acted_this_round: bool,
    dead: bool,
}

impl Unit {
    fn new(side: Side) -> Unit {
        Unit {
            side,
            hit_points: HIT_POINTS,
            acted_this_round: false,
            dead: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let cave = input_gen(include_str!("../input/tests/d15.txt"));
        assert_eq!(cave.units.len(), 9);
        assert_eq!(cave.elf_count(), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(include_str!("../input/tests/d15.txt")), 39514);
        assert_eq!(solve_part1(include_str!("../input/tests/d15-2.txt")), 18740);
        assert_eq!(solve_part1(include_str!("../input/tests/d15-3.txt")), 36334);
        assert_eq!(solve_part1(include_str!("../input/tests/d15-4.txt")), 28944);
    }

    #[test]
    fn test_part2() {
        let cave = input_gen(include_str!("../input/tests/d15-5.txt"));
        assert_eq!(cave.elf_count(), 2);
        assert_eq!(solve_part2(include_str!("../input/tests/d15-5.txt")), 4988);
    }
}
