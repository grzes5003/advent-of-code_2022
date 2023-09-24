use std::fmt;
use std::fmt::Formatter;
use crate::parser::parse;

type Point = (usize, usize);

struct Block {
    _points: Box<Vec<Point>>,
    _pos: Point,
}

struct Tower {
    _lines: [Vec<bool>; 7],
    _ground_offset: usize,
    _generator: Box<dyn Iterator<Item=Block>>,
    _moves: Box<dyn Iterator<Item=Move>>,
}

impl Tower {
    const MIN_OFFSET: usize = 3;

    fn fall(&mut self) {
        // println!("{:?}", self);
        // preparation
        self.level();
        let real_height = self.real_height();
        let mut block = self._generator.next().unwrap().move_to(&(2, real_height));
        self.add_levels(block.height());

        // println!("block: {:?}", block._points);
        // println!("real_height: {}", self.real_height());

        loop {
            let next_move = self._moves.next().unwrap();
            // println!("next_move: {:?}", next_move);
            let next_block = block.make_move(&next_move);
            if !self.is_colliding(&next_block) {
                block = next_block;
            }
            // println!("block;;;: {:?}", block._points);

            let next_block = block.make_move(&Move::Down);
            if self.is_colliding(&next_block) || block._points.iter().any(|(_, y)| *y == 0) {
                self.set_points(&block);
                if self.cycle_detection() {
                    println!("cycle_detection: {:?}", self.height());
                }
                self.optimise(&block);
                break;
            }
            block = next_block;
        }
    }

    fn new(start: Point, moves: Vec<Move>) -> Self {
        let mut lines: [Vec<bool>; 7] = Default::default();
        lines.iter_mut()
            .for_each(|line|
                line.append(Vec::from([false; 3].as_ref()).as_mut())
            );
        Tower {
            _lines: lines,
            _ground_offset: 0,
            _generator: Box::new(gen_blocks(start)),
            _moves: Box::new(moves.into_iter().cycle()),
        }
    }

    fn level(&mut self) {
        let height = self.height();
        let expected_height = height + Self::MIN_OFFSET;
        // println!("expected_height: {}, {}", expected_height, height);
        self._lines.iter_mut()
            .for_each(|line| {
                match line.len() {
                    len if len < expected_height => {
                        let diff = expected_height - len;
                        for _ in 0..diff {
                            line.push(false);
                        }
                    }
                    len if len > expected_height => {
                        let diff = len - expected_height;
                        for _ in 0..diff {
                            line.pop();
                        }
                    }
                    _ => {}
                }
            })
    }

    fn add_levels(&mut self, levels: usize) {
        self._lines.iter_mut()
            .for_each(|line| {
                for _ in 0..levels {
                    line.push(false);
                }
            })
    }

    fn height(&self) -> usize {
        self._lines.iter()
            .map(|line| self.line_height(line))
            .max().unwrap()
    }

    fn real_height(&self) -> usize {
        self._lines.iter()
            .map(|line| line.len())
            .max().unwrap() + self._ground_offset
    }

    fn line_height(&self, vec: &Vec<bool>) -> usize {
        (vec.iter()
            .rposition(|&b| b).map(|val| val as isize)
            .unwrap_or(-1) + 1) as usize + self._ground_offset
    }

    fn is_colliding(&self, block: &Block) -> bool {
        block._points.iter()
            .any(|(x, y)|
                *self._lines
                    .get(*x).unwrap_or(&vec![])
                    .get(*y).unwrap_or(&true)
            )
    }

    fn set_points(&mut self, block: &Block) {
        // println!("set_points: {:?}", block._points);
        block._points.iter()
            .for_each(|(x, y)| {
                self._lines[*x][*y] = true;
            });
    }

    fn optimise(&mut self, last_block: &Block) {
        for y in last_block._points.iter().map(|(_, y)| *y) {
            let is_filled = (0..7).into_iter()
                .map(|x| self._lines[x][y])
                .all(|val| val);
            if is_filled {
                self._lines.iter_mut()
                    .for_each(|line| {
                        line.drain(0..y);
                    });
                println!("optimising: {}", y);
                self._ground_offset += y;
                return;
            }
        }
    }

    fn cycle_detection(&self) -> bool {
        let height = self.height();
        if height % 2 != 0 {
            return false;
        }
        if !self._lines.iter()
            .all(|line| line.get(height / 2 - 1) == line.get(height - 1)) {
            return false;
        }

        self._lines.iter()
            .map(|line|
                line[0..(height/2)].iter()
                    .zip(line.iter().skip(height / 2))
                    .all(|(a, b)| a == b)
            ).all(|val| val)
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;
        for (x, y) in self._points.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        let mut result = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self._points.contains(&(x, y)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl fmt::Debug for Tower {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let height = self.real_height();
        for y in (0..height).rev() {
            for x in 0..7 {
                if self._lines[x][y] {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy)]
enum Type {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
    Down,
}

impl Move {
    fn from(s: &char) -> Result<Move, String> {
        match s {
            &'<' => Ok(Move::Left),
            &'>' => Ok(Move::Right),
            _ => Err("Unknown move".to_string())
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Move::Left => write!(f, "<"),
            Move::Right => write!(f, ">"),
            Move::Down => Ok(()),
        }
    }
}

impl Type {
    fn all() -> Vec<Type> {
        vec![Type::A, Type::B, Type::C, Type::D, Type::E]
    }
}

fn gen_blocks(start: Point) -> impl Iterator<Item=Block> {
    Type::all().into_iter().cycle()
        .map(move |tp| Block::of(&tp, start))
}

impl Block {
    fn of(tp: &Type, (x0, y0): Point) -> Self {
        let points = match tp {
            Type::A => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Type::B => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Type::C => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Type::D => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Type::E => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        };
        Block {
            _points: Box::new(points.into_iter().map(|(x, y)| (x + x0, y + y0)).collect()),
            _pos: (x0, y0),
        }
    }

    fn move_to(&self, point: &Point) -> Self {
        let (x0, y0) = self._pos;
        let (x1, y1) = point;
        let points = self._points.iter()
            .map(|(x, y)| (x + x1 - x0, y + y1 - y0))
            .collect::<Vec<_>>();
        Block {
            _points: Box::new(points),
            _pos: *point,
        }
    }

    fn make_move(&self, mv: &Move) -> Self {
        let (x0, y0) = self._pos;
        let points = self._points.iter()
            .map(|(x, y)| match mv {
                Move::Left => (x.checked_sub(1), Some(*y)),
                Move::Right => (match x + 1 {
                    val if val >= 7 => None,
                    val => Some(val)
                }, Some(*y)),
                Move::Down => (Some(*x), y.checked_sub(1)),
            })
            .map(|(x, y)| match (x, y) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>();
        if points.is_none() {
            let points = self._points.iter()
                .map(|(x, y)| (*x, *y))
                .collect::<Vec<_>>();
            // println!("points: {:?}", points);
            return Block {
                _points: Box::new(points),
                _pos: (x0, y0),
            };
        }
        Block {
            _points: Box::new(points.unwrap()),
            _pos: match mv {
                Move::Left => (x0 - 1, y0),
                Move::Right => (x0 + 1, y0),
                Move::Down => (x0, y0 - 1),
            },
        }
    }

    fn height(&self) -> usize {
        let max = self._points.iter().map(|(_, y)| y).max().unwrap();
        let min = self._points.iter().map(|(_, y)| y).min().unwrap();
        // println!("{:?}", self);
        // println!("max: {}, min: {}", max, min);
        max - min + 1
    }
}

fn task01(input: Vec<Move>) -> u64 {
    let mut tower = Tower::new((0, 0), input);
    for _ in 0i64..2022 {
        tower.fall();
    }
    tower.height() as u64
}

fn task02(input: Vec<Move>) -> u64 {
    let mut tower = Tower::new((0, 0), input);
    for i in 0i64..1000000000000 {
        if i % 1000000 == 0 {
            println!("i: {}, real_height: {}", i, tower.real_height());
        }
        tower.fall();
    }
    tower.height() as u64
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Move>, String> {
    input.get(0).ok_or("Not found input".to_string())?
        .chars().map(|ch| Move::from(&ch)).collect::<Result<Vec<Move>, String>>()
}

fn input_data() -> Vec<String> {
    parse("resources/day17_test.in")
}

pub fn wrapper_task02() {
    let input_data = input_data();
    let valves = parse_input(&input_data).expect("Error parse input");
    println!("task02: {}", task02(valves));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks() {
        let blocks = gen_blocks((0, 0)).take(5).collect::<Vec<Block>>();
        for i in blocks {
            println!("{:?}", i);
        }
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        println!("task01: {}", task01(valves));

        Ok(())
    }
}