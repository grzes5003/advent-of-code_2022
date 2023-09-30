use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;
use std::str::FromStr;
use crate::parser::parse;

type Dist = i32;
type Point = (Dist, Dist);

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => {
                match self {
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                }
            }
            Turn::Right => {
                match self {
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Left => Direction::Up,
                }
            }
        }
    }

    fn value(&self) -> Dist {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(Dist),
    Turn(Turn),
}

impl Instruction {
    fn vec_from_str(s: &str) -> Result<Vec<Self>, String> {
        let mut chars = s.chars();
        let mut res = Vec::new();

        let mut buffer = String::new();
        while let Some(ch) = chars.next() {
            if !ch.is_digit(10) {
                if buffer.len() > 0 {
                    res.push(Self::from_str(&buffer)?);
                }
                res.push(Self::from_str(&ch.to_string())?);
                buffer.clear();
            } else {
                buffer.push(ch);
            }
        }
        if buffer.len() > 0 {
            res.push(Self::from_str(&buffer)?);
        }
        Ok(res)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Instruction::Turn(Turn::Left)),
            "R" => Ok(Instruction::Turn(Turn::Right)),
            _ => {
                let dist = s.parse::<Dist>().map_err(|e| e.to_string())?;
                Ok(Instruction::Move(dist))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    position: Point,
    direction: Direction,
}

impl Player {
    fn next_pos(&self) -> Point {
        let (x, y) = self.position;
        match self.direction {
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
        }
    }
}

struct World {
    player: Player,
    map: HashMap<Point, bool>,
    instructions: Box<dyn Iterator<Item=Instruction>>,
}

impl World {
    fn step(&mut self) -> Option<Point> {
        let instruction = self.instructions.next()?;
        match instruction {
            Instruction::Move(dist) => {
                self.move_player(dist).ok()?;
            }
            Instruction::Turn(turn) => {
                self.player.direction = self.player.direction.turn(turn);
            }
        }
        Some(self.player.position)
    }

    fn move_player(&mut self, dist: Dist) -> Result<(), String> {
        let mut dist = dist;
        while dist > 0 {
            match self.player.next_pos() {
                point if self.map.contains_key(&point) => {
                    if self.map.get(&point) == Some(&true) {
                        self.player.position = point;
                    } else {
                        return Ok(());
                    }
                }
                _ => {
                    let point = self.find_cont()
                        .ok_or("Cannot find cont point")?.to_owned();
                    match self.map.get(&point) {
                        None => {
                            return Ok(());
                        }
                        Some(free) => {
                            if *free {
                                self.player.position = point;
                            } else {
                                return Ok(());
                            }
                        }
                    }
                }
            }
            dist -= 1;
        }
        Ok(())
    }

    fn find_cont(&self) -> Option<Point> {
        match self.player.direction {
            Direction::Down => {
                let (x, _) = self.player.position;
                self.map.iter()
                    .filter(|((a, _), _)| *a == x)
                    .min_by(|((_, y), _), ((_, b), _)| {
                        y.cmp(b)
                    }).map(|((x, y), _)| (*x, *y))
            }
            Direction::Right => {
                let (_, y) = self.player.position;
                self.map.iter()
                    .filter(|((_, b), _)| *b == y)
                    .min_by(|((x, _), _), ((a, _), _)| {
                        x.cmp(a)
                    }).map(|((x, y), _)| (*x, *y))
            }
            Direction::Up => {
                let (x, _) = self.player.position;
                self.map.iter()
                    .filter(|((a, _), _)| *a == x)
                    .max_by(|((_, y), _), ((_, b), _)| {
                        y.cmp(b)
                    }).map(|((x, y), _)| (*x, *y))
            }
            Direction::Left => {
                let (_, y) = self.player.position;
                self.map.iter()
                    .filter(|((_, b), _)| *b == y)
                    .max_by(|((x, _), _), ((a, _), _)| {
                        x.cmp(a)
                    }).map(|((x, y), _)| (*x, *y))
            }
        }
    }

    fn simulate(&mut self) -> Option<Player> {
        while self.step().is_some() {}
        Some(self.player.clone())
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for (x, y) in self.map.keys() {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = (x, y);
                if point == self.player.position {
                    write!(f, "x")?;
                } else {
                    match self.map.get(&point) {
                        Some(true) => write!(f, ".")?,
                        Some(false) => write!(f, "#")?,
                        None => write!(f, " ")?,
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn task01(world: &mut World) -> Option<Dist> {
    let player = world.simulate()?;
    println!("{:?} {:?}", player.position, player.direction);
    Some(4 * (player.position.0 + 1) + 1000 * (player.position.1 + 1) + player.direction.value())
}

fn parse_input(input: &Vec<String>) -> Result<World, String> {
    let mut map = HashMap::new();
    let mut player_start = (0, 0);

    if let [a, b] = input.split(|line| line.is_empty())
        .collect::<Vec<_>>().as_slice() {
        a.iter().enumerate()
            .for_each(|(idy, line)| {
                line.chars().enumerate()
                    .for_each(|(idx, ch)| {
                        let point = (idx as Dist, idy as Dist);
                        match ch {
                            '#' => { map.insert(point, false); }
                            '.' => {
                                if map.len() == 0 {
                                    player_start = point;
                                }
                                map.insert(point, true);
                            }
                            _ => {}
                        };
                    })
            });

        if b.len() != 1 {
            return Err("Invalid second part of input".to_string());
        }
        let b = b.get(0).unwrap();
        let instructions = Instruction::vec_from_str(&b)?.into_iter();
        let player = Player {
            position: player_start,
            direction: Direction::Right,
        };
        return Ok(World {
            player,
            map,
            instructions: Box::new(instructions),
        });
    }
    Err("Invalid input".to_string())
}

fn input_data() -> Vec<String> {
    parse("resources/day22.in")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> World {
        let input_data = parse("resources/day22_test.in");
        let mut world = parse_input(&input_data).unwrap();
        world.instructions = Box::new(vec![Instruction::Move(1)].into_iter());
        world
    }

    #[test]
    fn world_test01() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (5, 4);
        world.player.direction = Direction::Up;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (5, 7));
        Ok(())
    }


    #[test]
    fn world_test02() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (11, 6);
        world.player.direction = Direction::Right;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (0, 6));
        Ok(())
    }


    #[test]
    fn world_test03() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (5, 5);
        world.player.direction = Direction::Right;
        world.instructions = Box::new(vec![Instruction::Move(10)].into_iter());

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (7, 5));
        Ok(())
    }

    #[test]
    fn world_test_wrap_wall_01() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (3, 7);
        world.player.direction = Direction::Down;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (3, 7));
        Ok(())
    }

    #[test]
    fn world_test_wrap_wall_02() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (11, 2);
        world.player.direction = Direction::Right;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (11, 2));
        Ok(())
    }

    #[test]
    fn world_test_wrap_wall_03() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (8, 0);
        world.player.direction = Direction::Left;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (8, 0));
        Ok(())
    }

    #[test]
    fn world_test_wrap_wall_04() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (14, 8);
        world.player.direction = Direction::Up;

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (14, 8));
        Ok(())
    }


    #[test]
    fn world_test05() -> Result<(), String> {
        let mut world = setup();

        world.player.position = (5, 4);
        world.player.direction = Direction::Down;
        world.instructions = Box::new(Instruction::vec_from_str("11L")?.into_iter());
        println!("{:?}", Instruction::vec_from_str("11L"));

        println!("{:?}", world);
        task01(&mut world);
        println!("{:?}", world);

        assert_eq!(world.player.position, (5, 7));
        Ok(())
    }

    #[test]
    fn world_test() -> Result<(), String> {
        let input_data = parse("resources/day22_test.in");
        let mut world = parse_input(&input_data)?;

        assert_eq!(task01(&mut world), Some(6032));
        Ok(())
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let mut world = parse_input(&input_data)?;7
        println!("task01: {:?}", task01(&mut world));

        Ok(())
    }
}