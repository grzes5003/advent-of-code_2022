use std::fmt;
use std::fmt::Formatter;

type Pair = (usize, usize);

#[derive(Debug)]
struct Line(Vec<Pair>);

struct Map(Vec<Vec<u8>>);

#[derive(Debug)]
struct Sand(Pair);

impl Map {
    fn new(lines: Vec<Line>) -> Map {
        let max_x = lines.iter().flat_map(|line| line.max_x()).max().unwrap();
        let max_y = lines.iter().flat_map(|line| line.max_y()).max().unwrap();

        let mut map = vec![vec![0u8; max_x + 1]; max_y + 1];

        for line in lines {
            line.0.iter().zip(line.0.iter().skip(1))
                .for_each(|((x1, y1), (x2, y2))| {
                    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
                    let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
                    for x in *x1..=*x2 {
                        map[*y1][x] = 1;
                    }
                    for y in *y1..=*y2 {
                        map[y][*x2] = 1;
                    }
                });
        }

        Map(map)
    }

    fn new_with_floor(lines: Vec<Line>) -> Map {
        let max_x = lines.iter().flat_map(|line| line.max_x()).max().unwrap();
        let max_y = lines.iter().flat_map(|line| line.max_y()).max().unwrap();
        println!("max_x: {}, max_y: {}", max_x, max_y);
        let new_lines = lines.into_iter()
            .chain(vec![Line(vec![(0, max_y + 2), (max_x*2, max_y + 2)])])
            .collect::<Vec<_>>();
        Map::new(new_lines)
    }

    fn occupied(&self, pair: &Pair) -> bool {
        !self.out(pair) && self.0[pair.1][pair.0] > 0
    }

    fn out(&self, pair: &Pair) -> bool {
        pair.0 >= self.0[0].len() || pair.1 >= self.0.len()
    }

    fn mark(&mut self, pair: &Pair) {
        self.0[pair.1][pair.0] = 2;
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, line) in self.0.iter().enumerate() {
            write!(f, "{:03}: ", idx)?;
            for c in line {
                write!(f, "{}", match *c {
                    1 => { '#' }
                    2 => { 'O' }
                    0 => { '.' }
                    _ => { '?' }
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Line {
    fn max_x(&self) -> Option<usize> {
        self.0.iter().map(|pair| pair.0).max()
    }

    fn max_y(&self) -> Option<usize> {
        self.0.iter().map(|pair| pair.1).max()
    }
}

impl Sand {
    fn new(pair: Pair) -> Sand {
        Sand(pair)
    }

    fn transform(self, map: &Map) -> Option<Sand> {
        let mut sand = self;
        loop {
            if map.out(&sand.0) {
                return None;
            }

            match (sand.0.0, sand.0.1) {
                (x, y) if !map.occupied(&(x, y + 1)) => {
                    sand = Sand((x, y + 1));
                }
                (x, y) if !map.occupied(&(x - 1, y + 1)) => {
                    sand = Sand((x - 1, y + 1));
                }
                (x, y) if !map.occupied(&(x + 1, y + 1)) => {
                    sand = Sand((x + 1, y + 1));
                }
                _ => return Some(sand)
            }
        }
    }
}

fn task01(map: &mut Map) -> u16 {
    let mut counter = 0;
    while let Some(sand) = Sand::new((500, 0)).transform(&map) {
        map.mark(&sand.0);
        counter += 1;
    }
    counter
}

fn task02(map: &mut Map) -> u32 {
    let mut counter = 0u32;
    while let Some(sand) = Sand::new((500, 0)).transform(&map) {
        map.mark(&sand.0);
        counter += 1;
        if sand.0 == (500, 0) {
            break;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn parse_input(input: Vec<String>) -> Result<Vec<Line>, String> {
        input.into_iter()
            .map(|line| {
                let coords = line.split("->")
                    .map(|part| part.trim().split(',').collect::<Vec<&str>>())
                    .map(|part| {
                        if let [a, b] = part.as_slice() {
                            let a_parsed = a.parse::<usize>()
                                .map_err(|err| err.to_string())?;
                            let b_parsed = b.parse::<usize>()
                                .map_err(|err| err.to_string())?;
                            Ok((a_parsed, b_parsed))
                        } else {
                            Err("Invalid input".to_owned())
                        }
                    }).collect::<Result<Vec<Pair>, String>>();
                match coords {
                    Ok(coords) => Ok(Line(coords)),
                    Err(err) => Err(err)
                }
            }).collect()
    }

    fn input_data() -> Vec<String> {
        parse("resources/day14.in")
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_pairs = parse_input(input_data())?;
        let mut map = Map::new(input_pairs);
        let res = task01(&mut map);
        println!("{:?}", map);
        println!("task01: {}", res);

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_pairs = parse_input(input_data())?;
        let mut map = Map::new_with_floor(input_pairs);
        let res = task02(&mut map);
        println!("{:?}", map);
        println!("task01: {}", res);

        Ok(())
    }
}