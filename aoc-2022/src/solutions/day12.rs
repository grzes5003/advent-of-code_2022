use std::collections::{HashMap, VecDeque};

type Point = (usize, usize);
type PointI16 = (i16, i16);

struct Map {
    _map: Vec<Vec<u8>>,
    start: Point,
    end: Point,
}

impl Map {
    fn bfs(&self) -> Option<u16> {
        let mut parent = HashMap::<Point, Point>::new();

        let mut visited = vec![vec![false; self._map[0].len()]; self._map.len()];
        visited[self.start.1][self.start.0] = true;
        let mut queue = VecDeque::new();
        queue.push_back(self.start);

        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            if point == self.end {
                return Some(self.backtrace_len(parent, point));
            }

            for neigh in self.neighbours(point) {
                if !visited[neigh.1][neigh.0] {
                    visited[neigh.1][neigh.0] = true;
                    parent.insert(neigh, point);
                    queue.push_back(neigh);
                }
            }
        }
        None
    }

    fn backtrace_len(&self, parent: HashMap<Point, Point>, org_point: Point) -> u16 {
        let mut path_len = 0;
        let mut point = org_point;

        while point != self.start {
            point = *parent.get(&point).unwrap();
            path_len += 1;
        }
        path_len
    }

    fn neighbours(&self, point: Point) -> Vec<Point> {
        let mut neighbours =
            vec![(point.0 as i16, point.1 as i16 - 1),
                 (point.0 as i16 + 1, point.1 as i16),
                 (point.0 as i16, point.1 as i16 + 1),
                 (point.0 as i16 - 1, point.1 as i16)];

        neighbours.into_iter()
            .filter(|a| !self.is_outside(a) && self.can_go(&point, a))
            .map(|a| (a.0 as usize, a.1 as usize))
            .collect::<Vec<_>>()
    }

    fn can_go(&self, a: &Point, b: &PointI16) -> bool {
        let a_val = self.get(a);
        let b_val = self.get(&(b.0 as usize, b.1 as usize));

        if b_val == 69 {
            let hen = self.highest_end_neighbours();
            return hen.contains(&self.get(a))
        }

        if a_val == 83 {
            return b_val == 97 || b_val == 98;
        }

        b_val.checked_sub(a_val) <= Some(1) || a_val == 69
    }

    fn is_outside(&self, point: &PointI16) -> bool {
        if point.0 < 0 || point.1 < 0 {
            return true;
        }
        if point.0 >= self._map[0].len() as i16 || point.1 >= self._map.len() as i16 {
            return true;
        }
        false
    }

    fn highest_end_neighbours(&self) -> Vec<u8> {
        let neighbours = self.neighbours(self.end);
        let max = neighbours.iter()
            .map(|a| self.get(a))
            .max().unwrap();

        neighbours
            .into_iter()
            .map(|a| self.get(&a))
            .filter(|a| *a == max)
            .collect::<Vec<_>>()
    }

    fn get(&self, point: &Point) -> u8 {
        self._map[point.1][point.0]
    }
}

fn parse_input(input: Vec<String>) -> Map {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => ()
            }
        }
    }

    let map = input.into_iter()
        .map(|line|
            line.as_bytes().to_vec())
        .collect();

    Map {
        _map: map,
        start,
        end
    }
}

fn task01(input: Vec<String>) -> u16 {
    let map = parse_input(input);
    map.bfs().unwrap()
}

fn replace_s(input: &mut Vec<String>, last: Point) -> Option<Point> {
    for y in last.1..input.len() {
        for x in 0..input[0].len() {
            if y == last.1 && x <= last.0 {
                continue;
            }
            if input[y].chars().nth(x).unwrap() == 'a' {
                let mut line = input[y].clone();
                line.replace_range(x..x + 1, "S");
                input[y] = line;
                return Some((x, y));
            }
        }
    }
    None
}

fn task02(input: Vec<String>) -> u16 {
    let mut input = input;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y].chars().nth(x).unwrap() == 'S' {
                let mut line = input[y].clone();
                line.replace_range(x..x + 1, "a");
                input[y] = line;
                break;
            }
        }
    }

    let mut results = Vec::new();
    let mut last = (0, 0);
    loop {
        let mut input = input.clone();
        match replace_s(&mut input, last) {
            Some(point) => {
                last = point;
                let map = parse_input(input);
                let res = map.bfs().unwrap_or(u16::MAX);
                results.push(res);
            },
            None => break
        }
    }
    *results.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day12.in")
    }

    #[test]
    fn task01_test() {
        let input = input_data();
        let res = task01(input);
        println!("task01: {}", res)
    }

    #[test]
    fn task02_test() {
        let input = input_data();
        let res = task02(input);
        println!("task01: {}", res)
    }
}