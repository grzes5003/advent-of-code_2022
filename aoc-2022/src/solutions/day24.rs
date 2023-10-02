use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use crate::parser::parse;

type Val = isize;
type Point = (Val, Val);


#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    point_zero: Point,
    direction: Direction,
}

impl Blizzard {
    fn pos_at(&self, time: Val, (w, h): Point) -> Point {
        let (x, y) = self.point_zero;
        match self.direction {
            Direction::Up => {
                (x, (y - time - 1).rem_euclid(h - 2) + 1)
            }
            Direction::Down => {
                (x, (y + time - 1).rem_euclid(h - 2) + 1)
            }
            Direction::Left => {
                ((x - time - 1).rem_euclid(w - 2) + 1, y)
            }
            Direction::Right => {
                ((x + time - 1).rem_euclid(w - 2) + 1, y)
            }
        }
    }
}

struct Plain {
    blizzards: Vec<Blizzard>,
    walls: Point,
}

impl fmt::Debug for Plain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (w, h) = self.walls;
        for y in 0..h {
            for x in 0..w {
                let ch = if (x, y) == (1, 0) || (x, y) == (w - 2, h - 1) {
                    'S'
                } else if self.blizzards.iter().any(|blizzard| blizzard.point_zero == (x, y)) {
                    'X'
                } else {
                    '.'
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Plain {
    fn plain_at(&self, time: Val) -> HashSet<Point> {
        let (w, h) = self.walls;
        self.blizzards.iter()
            .map(|blizzard| blizzard.pos_at(time, (w, h)))
            .collect()
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let (x, y) = point;
        vec![
            (x + 1, *y),
            (x - 1, *y),
            (*x, y + 1),
            (*x, y - 1),
            (*x, *y),
        ]
    }

    fn possible_moves_at(&self, time: Val, point: &Point) -> Vec<Point> {
        let plain_set = self.plain_at(time);
        self.neighbors(point).into_iter()
            .filter(|point| !plain_set.contains(point))
            .filter(|point| {
                let (x, y) = point;
                let (w, h) = self.walls;
                (*x > 0 && *y > 0 && *x < w - 1 && *y < h - 1)
                    || (*x == w - 2 && *y == h - 1)
                    || (*x == 1 && *y == 0)
            }).collect()
    }

    fn bfs(&self, start_pos: Point, target_pos: Point, start_time: Val) -> Option<Val> {
        let mut cache = HashSet::new();
        let mut queue = VecDeque::from(vec![(start_time, start_pos)]);

        while let Some((time, point)) = queue.pop_front() {
            if cache.contains(&(time, point)) {
                continue;
            }
            cache.insert((time, point));
            let time = time + 1;
            for next_point in self.possible_moves_at(time, &point) {
                if next_point == target_pos {
                    return Some(time);
                }
                queue.push_back((time, next_point));
            }
        }
        None
    }
}

fn task01(plain: Plain) -> Option<Val> {
    let (w, h) = plain.walls;
    plain.bfs((1, 0), (w - 2, h - 1), 0)
}

fn task02(plain: Plain) -> Option<Val> {
    let (w, h) = plain.walls;
    let mut time = plain.bfs((1, 0), (w - 2, h - 1), 0)?;
    time = plain.bfs((w - 2, h - 1), (1, 0), time)?;
    time = plain.bfs((1, 0), (w - 2, h - 1), time)?;
    Some(time)
}

fn parse_input(input: &Vec<String>) -> Plain {
    let height = input.len();
    let width = input[0].len();

    let blizzards = input.into_iter().enumerate()
        .map(|(ixy, line)| {
            line.chars().enumerate().map(|(idx, ch)| {
                let idx = idx as Val;
                let ixy = ixy as Val;
                match ch {
                    '<' => {
                        Some(Blizzard {
                            point_zero: (idx, ixy),
                            direction: Direction::Left,
                        })
                    }
                    '>' => {
                        Some(Blizzard {
                            point_zero: (idx, ixy),
                            direction: Direction::Right,
                        })
                    }
                    '^' => {
                        Some(Blizzard {
                            point_zero: (idx, ixy),
                            direction: Direction::Up,
                        })
                    }
                    'v' => {
                        Some(Blizzard {
                            point_zero: (idx, ixy),
                            direction: Direction::Down,
                        })
                    }
                    _ => None
                }
            }).flatten().collect::<Vec<_>>()
        }).flat_map(|item| item).collect::<Vec<_>>();

    Plain {
        blizzards,
        walls: (width as Val, height as Val),
    }
}

fn input_data() -> Vec<String> {
    parse("resources/day24.in")
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn plain_test() {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("{:?}", valves);

        for i in 0..10 {
            let plain_set = valves.plain_at(i);
            let pl = Plain {
                blizzards: plain_set.iter().map(|point| Blizzard {
                    point_zero: *point,
                    direction: Direction::Up,
                }).collect(),
                walls: valves.walls,
            };
            println!("{}", i);
            println!("{:?}", pl);
        }
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("task01: {:?}", task01(valves));

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("task02: {:?}", task02(valves));

        Ok(())
    }
}