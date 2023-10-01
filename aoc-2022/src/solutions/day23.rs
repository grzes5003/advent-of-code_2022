use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use itertools::Itertools;
use crate::parser::parse;

type Id = usize;
type Val = isize;
type Point = (Val, Val);

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![Direction::North, Direction::South, Direction::West, Direction::East]
    }

    fn offset(offset: u8) -> Vec<Self> {
        Self::all().into_iter().cycle()
            .skip(offset as usize)
            .take(4)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Elf(Id, Point, u8);

impl Elf {
    fn move_to(&mut self, point: &Point) {
        self.1 = point.clone();
    }

    fn shift_offset(self, offset: u8) -> Self {
        Elf {
            0: self.0,
            1: self.1,
            2: (self.2 + offset).rem_euclid(4),
        }
    }
}

struct Plain {
    elfs: HashMap<Point, Elf>,
}

impl fmt::Debug for Plain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let y_max = self.elfs.iter()
            .max_by(|(a, _), (b, _)| a.1.cmp(&b.1))
            .unwrap().0.1;
        let y_min = self.elfs.iter()
            .min_by(|(a, _), (b, _)| a.1.cmp(&b.1))
            .unwrap().0.1;
        let x_max = self.elfs.iter()
            .max_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .unwrap().0.0;
        let x_min = self.elfs.iter()
            .min_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .unwrap().0.0;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if let Some(elf) = self.elfs.get(&(x, y)) {
                    // write!(f, "[{}]", elf.0)?;
                    write!(f, "#")?;
                } else {
                    // write!(f, " . ")?;
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Plain {
    fn from_elfs(elfs: &Vec<Elf>) -> Self {
        let map = elfs.iter()
            .map(|elf| (elf.1, elf.clone()))
            .collect::<HashMap<Point, Elf>>();
        Plain { elfs: map }
    }

    fn has_neighbour(&self, point: &Point) -> bool {
        vec![
            (point.0, point.1 - 1),
            (point.0 - 1, point.1 - 1),
            (point.0 + 1, point.1 - 1),
            (point.0, point.1 + 1),
            (point.0 - 1, point.1 + 1),
            (point.0 + 1, point.1 + 1),
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
        ].iter()
            .any(|point| self.elfs.get(point).is_some())
    }

    fn occupied(&self, point: &Point, direction: &Direction) -> Option<Direction> {
        (match direction {
            Direction::North => {
                vec![
                    (point.0, point.1 - 1),
                    (point.0 - 1, point.1 - 1),
                    (point.0 + 1, point.1 - 1),
                ]
            }
            Direction::South => {
                vec![
                    (point.0, point.1 + 1),
                    (point.0 - 1, point.1 + 1),
                    (point.0 + 1, point.1 + 1),
                ]
            }
            Direction::West => {
                vec![
                    (point.0 - 1, point.1),
                    (point.0 - 1, point.1 - 1),
                    (point.0 - 1, point.1 + 1),
                ]
            }
            Direction::East => {
                vec![
                    (point.0 + 1, point.1),
                    (point.0 + 1, point.1 - 1),
                    (point.0 + 1, point.1 + 1),
                ]
            }
        }).iter()
            .all(|point| self.elfs.get(point).is_none())
            .then(|| direction.clone())
    }

    fn stage(&self) -> Vec<Elf> {
        let mut staged = Vec::new();
        for elf in self.elfs.values() {
            if !self.has_neighbour(&elf.1) {
                continue;
            }
            if let Some(dir) = Direction::offset(elf.2).iter()
                .find_map(|direction| self.occupied(&elf.1, direction)) {
                let elf = Elf {
                    0: elf.0,
                    1: match dir {
                        Direction::North => (elf.1.0, elf.1.1 - 1),
                        Direction::South => (elf.1.0, elf.1.1 + 1),
                        Direction::West => (elf.1.0 - 1, elf.1.1),
                        Direction::East => (elf.1.0 + 1, elf.1.1),
                    },
                    2: elf.2,
                };
                staged.push(elf);
            }
        }
        staged
    }

    fn update(&mut self, staged: &Vec<Elf>) -> Option<()> {
        let updated = staged.iter()
            .into_group_map_by(|elf| elf.1)
            .into_iter()
            .map(|(_, group)| group)
            .filter(|group| group.len() == 1)
            .map(|group| group[0].clone())
            .map(|elf| (elf.0, elf))
            .collect::<HashMap<Id, Elf>>();
        if updated.is_empty() {
            return None;
        }
        let rest = self.elfs.iter()
            .map(|(_, elf)| (elf.0, elf.clone()))
            .filter(|(id, _)| !updated.contains_key(id))
            .map(|(id, elf)| (id, elf))
            .collect::<HashMap<Id, Elf>>();
        self.elfs = updated.into_iter().chain(rest.into_iter())
            .map(|(_, elf)|
                (elf.1, elf.shift_offset(1))).collect::<HashMap<Point, _>>();
        Some(())
    }

    fn step(&mut self) -> Option<()> {
        let staged = self.stage();
        self.update(&staged)
    }

    fn find_free_fields(&self) -> Val {
        let y_max = self.elfs.iter()
            .max_by(|(a, _), (b, _)| a.1.cmp(&b.1))
            .unwrap().0.1;
        let y_min = self.elfs.iter()
            .min_by(|(a, _), (b, _)| a.1.cmp(&b.1))
            .unwrap().0.1;
        let x_max = self.elfs.iter()
            .max_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .unwrap().0.0;
        let x_min = self.elfs.iter()
            .min_by(|(a, _), (b, _)| a.0.cmp(&b.0))
            .unwrap().0.0;
        (y_max - y_min + 1).abs() * (x_max - x_min + 1).abs() - self.elfs.len() as Val
    }
}

fn task01(elfs: Vec<Elf>) -> Val {
    let mut plain = Plain::from_elfs(&elfs);
    for _ in 0..10 {
        plain.step();
    }
    plain.find_free_fields()
}

fn task02(elfs: Vec<Elf>) -> Val {
    let mut plain = Plain::from_elfs(&elfs);
    let mut counter = 1;
    while let Some(_) = plain.step() {
        counter += 1;
    }
    counter
}


fn parse_input(input: &Vec<String>) -> Vec<Elf> {
    let mut counter = 0;
    input.iter().enumerate()
        .map(|(idy, line)| {
            line.chars().enumerate().map(|(idx, ch)| {
                match ch {
                    '#' => {
                        counter += 1;
                        Some(Elf(counter, (idx as Val, idy as Val), 0u8))
                    }
                    _ => None
                }
            }).flatten().collect::<Vec<_>>()
        }).flat_map(|item| item).collect::<Vec<Elf>>()
}

fn input_data() -> Vec<String> {
    parse("resources/day23.in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("task01: {}", task01(valves));

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("task02: {}", task02(valves));

        Ok(())
    }
}