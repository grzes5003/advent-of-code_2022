use std::collections::{HashMap, HashSet};
use std::isize;
use crate::parser::parse;

type Point = (isize, isize, isize);
type BS = (Point, Side);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Side {
    pub fn iter() -> impl Iterator<Item=Self> {
        let sides = vec![
            Side::Up,
            Side::Down,
            Side::Left,
            Side::Right,
            Side::Front,
            Side::Back,
        ];
        sides.into_iter()
    }
}

struct Block {
    sides: u8,
}


fn adjacent((x, y, z): Point) -> Vec<(Side, Point)> {
    vec![
        (Side::Left, (x - 1, y, z)),
        (Side::Right, (x + 1, y, z)),
        (Side::Down, (x, y - 1, z)),
        (Side::Up, (x, y + 1, z)),
        (Side::Back, (x, y, z - 1)),
        (Side::Front, (x, y, z + 1)),
    ]
}

fn mirror(side: &Side) -> Side {
    match side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
        Side::Down => Side::Up,
        Side::Up => Side::Down,
        Side::Back => Side::Front,
        Side::Front => Side::Back,
    }
}

fn decode(side: &Side, val: u8) -> bool {
    match side {
        Side::Left => val & 0b00000001 == 0b00000001,
        Side::Right => val & 0b00000010 == 0b00000010,
        Side::Down => val & 0b00000100 == 0b00000100,
        Side::Up => val & 0b00001000 == 0b00001000,
        Side::Back => val & 0b00010000 == 0b00010000,
        Side::Front => val & 0b00100000 == 0b00100000,
    }
}

fn encode(side: &Side, val: u8) -> u8 {
    match side {
        Side::Left => val | 0b00000001,
        Side::Right => val | 0b00000010,
        Side::Down => val | 0b00000100,
        Side::Up => val | 0b00001000,
        Side::Back => val | 0b00010000,
        Side::Front => val | 0b00100000,
    }
}

fn calculate(input: &Vec<Point>) -> HashMap::<Point, u8> {
    let mut blocks = HashMap::<Point, u8>::new();

    for point in input {
        let adj = adjacent(*point);
        let mut blocked = 0b00000000u8;
        for (side, xyz) in adj {
            if let Some(val) = blocks.get(&xyz) {
                blocked = encode(&side, blocked);
                *blocks.get_mut(&xyz).unwrap() = encode(&mirror(&side), *val);
            }
        }
        blocks.insert(*point, blocked);
    }
    blocks
}

fn is_out(point: &Point, limits: &(Point, Point)) -> bool {
    let (x_max, y_max, z_max) = limits.1;
    let (x_min, y_min, z_min) = limits.0;
    point.0 < x_min || point.0 > x_max || point.1 < y_min || point.1 > y_max || point.2 < z_min || point.2 > z_max
}

fn expand(point: Point, created: &mut HashSet<Point>, existing: &HashSet<Point>, area: &mut u64, limits: (Point, Point)) -> u64 {
    let mut stack = vec![point];

    while let Some(current_point) = stack.pop() {
        if created.contains(&current_point) {
            continue;
        }

        created.insert(current_point.clone());

        let adj = adjacent(current_point).iter()
            .filter_map(|(_, p)| {
                if is_out(p, &limits) || created.contains(p) {
                    None
                } else {
                    Some(*p)
                }
            }).collect::<Vec<Point>>();

        for p in adj {
            if !existing.contains(&p) {
                stack.push(p);
            } else {
                *area += 1;
            }
        }
    }

    *area
}

fn task01(input: Vec<Point>) -> u64 {
    let blocks = calculate(&input);

    blocks.values()
        .map(|val| val.count_zeros() as u64 - 2)
        .sum::<u64>()
}

fn task02(input: Vec<Point>) -> u64 {
    let xyz_max = input.iter().fold((isize::MIN, isize::MIN, isize::MIN),
                               |acc, &(x, y, z)|
                                   (acc.0.max(x), acc.1.max(y), acc.2.max(z)));

    let xyz_min = input.iter().fold((isize::MAX, isize::MAX, isize::MAX),
                                                  |acc, &(x, y, z)|
                                                      (acc.0.min(x), acc.1.min(y), acc.2.min(z)));
    let xyz_min = (xyz_min.0 - 1, xyz_min.1 - 1, xyz_min.2 - 1);
    let xyz_max = (xyz_max.0 + 1, xyz_max.1 + 1, xyz_max.2 + 1);

    expand(xyz_min, &mut HashSet::<Point>::new(), &input.into_iter().collect::<HashSet<Point>>(), &mut 0, (xyz_min, xyz_max))
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Point>, String> {
    input.into_iter()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().ok_or("x not found")?.parse::<isize>().map_err(|e| format!("x: {}", e))?;
            let y = parts.next().ok_or("y not found")?.parse::<isize>().map_err(|e| format!("y: {}", e))?;
            let z = parts.next().ok_or("z not found")?.parse::<isize>().map_err(|e| format!("z: {}", e))?;
            Ok((x, y, z))
        }).collect()
}

fn input_data() -> Vec<String> {
    parse("resources/day18.in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        println!("task01: {}", task01(valves));

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        println!("task02: {}", task02(valves));

        Ok(())
    }
}
