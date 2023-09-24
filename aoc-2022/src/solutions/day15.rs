use std::collections::HashSet;
use std::future::Future;
use std::sync::Arc;
use fancy_regex::Regex;
use futures::{FutureExt, StreamExt};
use tokio::runtime::Builder;

type Size = i64;
type Point = (Size, Size);

#[derive(Debug, Clone)]
pub struct Sonar {
    x: Size,
    y: Size,
    radius: Size,
}

impl Sonar {
    const LIMIT: Size = 4000000;
    fn boundary(&self) -> (Size, Size, Size, Size) {
        (self.x - self.radius, self.x + self.radius, self.y - self.radius, self.y + self.radius)
    }

    fn contains(&self, point: &Point) -> bool {
        manhhatan_distance(&(self.x, self.y), point) <= self.radius
    }

    fn boundary_lines(&self) -> impl Iterator<Item=Point> + '_ {
        let radius = self.radius + 1;
        (0..radius)
            .map(move |i| (self.x + i, self.y + radius - i))
            .chain((0..radius).map(move |i| (self.x - radius + i, self.y + i)))
            .chain((0..radius).map(move |i| (self.x + radius - i, self.y - i)))
            .chain((0..radius).map(move |i| (self.x - i, self.y - radius + i)))
            .filter(|point| Sonar::in_search_area(point))
    }

    fn in_search_area(point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 <= Self::LIMIT && point.1 <= Self::LIMIT
    }
}

fn manhhatan_distance(a: &Point, b: &Point) -> Size {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_coord(cap: &fancy_regex::Captures, index: usize) -> Result<Size, String> {
    cap.get(index)
        .ok_or("Not found".to_string())?.as_str().parse::<Size>()
        .map_err(|err| err.to_string())
}

fn find_edges(input: &Vec<Sonar>) -> (Size, Size, Size, Size) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for sonar in input {
        let (x_min, x_max, y_min, y_max) = sonar.boundary();
        if x_min < min_x {
            min_x = x_min;
        }
        if x_max > max_x {
            max_x = x_max;
        }
        if y_min < min_y {
            min_y = y_min;
        }
        if y_max > max_y {
            max_y = y_max;
        }
    }
    (min_x, max_x, min_y, max_y)
}

fn task01(sonars: Vec<Sonar>, beacons: HashSet<Point>) -> Size {
    let line_y = Sonar::LIMIT;
    let (min_x, max_x, _, _) = find_edges(&sonars);
    println!("{} {}", min_x, max_x);
    (min_x..=max_x)
        .into_iter()
        .filter(|idx|
            sonars.iter().any(|sonar| sonar.contains(&(*idx, line_y)))
                && !beacons.contains(&(*idx, line_y))
        )
        .count() as Size
}

async fn search_range(sonars: Arc<Vec<Sonar>>, start: Size, size: Size) -> Option<Size> {
    for y in start..=start + size {
        for x in start..=4000000 {
            if sonars.iter().all(|sonar| !sonar.contains(&(x, y))) {
                return Some(x * 4000000 + y);
            }
        }
    }
    return None;
}

async fn search_iter(sonars: Arc<Vec<Sonar>>, idx: usize) -> Option<Size> {
    let boundary = sonars.get(idx).unwrap().boundary_lines().collect::<Vec<_>>();
    for point in boundary {
        if sonars.iter().all(|sonar| !sonar.contains(&point)) {
            println!("result: {:?}", point);
            return Some(point.0 * 4000000 + point.1);
        }
    }
    None
}

pub async fn task02(sonars: Vec<Sonar>) -> Option<Size> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("workers")
        .build()
        .unwrap();

    let sonars_arc = Arc::new(sonars.clone());
    let mut fut = sonars.iter().enumerate()
        .map(|(idx,_)| {
            let sonars = Arc::clone(&sonars_arc);
            runtime.spawn(async move {
                search_iter(sonars, idx).await
            })
        }).collect::<Vec<_>>().into_iter();
    while let Some(result) = fut.next() {
        if let Ok(value) = result.await {
            if let Some(value) = value {
                runtime.shutdown_background();
                return Some(value)
            }
        }
    }
    None
}

pub fn parse_input(input: Vec<String>) -> Result<(Vec<Sonar>, HashSet<Point>), String> {
    let re = Regex::new(r"((?<=at x=)-?\d+).*?((?<=y=)-?\d+)").unwrap();
    let (sonars, points): (Vec<_>, Vec<_>) = input.into_iter()
        .map(|line| {
            let mut caps = re.captures_iter(&line);
            let first = caps.next().ok_or("Not found".to_string())?
                .map_err(|err| err.to_string())?;
            let x = get_coord(&first, 1)?;
            let y = get_coord(&first, 2)?;

            let second = caps.next().ok_or("Not found".to_string())?
                .map_err(|err| err.to_string())?;
            let i = get_coord(&second, 1)?;
            let j = get_coord(&second, 2)?;
            Ok((Sonar { x, y, radius: manhhatan_distance(&(x, y), &(i, j)) }, (i, j)))
        }).collect::<Result<Vec<(Sonar, Point)>, String>>()?
        .into_iter().unzip();

    Ok((sonars, HashSet::from_iter(points.into_iter())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day15_test.in")
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let (sonars, beacons) = parse_input(input_data())?;
        println!("task01: {}", task01(sonars, beacons));

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let (_, _) = parse_input(input_data())?;
        Ok(())
    }
}