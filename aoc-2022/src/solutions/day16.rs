use std::cmp;
use std::collections::{HashMap, HashSet};
use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Closed,
    Open,
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    id: &'a str,
    rate: u16,
    connected: Vec<&'a str>,
    status: Status,
}

impl<'a> Valve<'a> {
    const MAX_TIME: u8 = 30;

    fn re() -> Regex {
        Regex::new(r"^Valve\s(.{2}).*rate=(\d*).*valves?\s(.*$)").unwrap()
    }

    fn open(&mut self) {
        self.status = Status::Open;
    }

    fn close(&mut self) {
        self.status = Status::Closed;
    }

    fn from_str(input: &'a str) -> Result<Self, String> {
        let cap = Self::re().captures(input)
            .map_err(|err| err.to_string())?.ok_or("Not found".to_string())?;

        let rate = cap.get(2).ok_or("Not found rate".to_string())?.as_str().parse::<u16>()
            .map_err(|err| err.to_string())?;
        let connected = cap.get(3).ok_or("Not found connected".to_string())?.as_str()
            .split(", ").map(|s| s.trim()).collect::<Vec<_>>();
        Ok(Valve {
            id: cap.get(1).ok_or("Not found id".to_string())?.as_str(),
            rate,
            connected,
            status: Status::Closed,
        })
    }
}

fn parse_input<'a>(input: &'a Vec<String>) -> Result<Vec<Valve<'a>>, String> {
    input.into_iter()
        .map(move |line| Valve::<'a>::from_str(&line))
        .collect::<Result<Vec<Valve<'a>>, String>>()
}

fn explore(current: &str, map: &HashMap<&str, Valve>, time_left: u8, score: u32, opened: HashSet<&str>, mem: &mut HashMap<String, u32>) -> u32 {
    if let Some(&score) = mem.get(format!("{}:{}:{:?}", current, time_left, opened).as_str()) {
        return score;
    }

    if time_left == 0 || opened.len() == map.len() {
        return score;
    }

    let valve = map.get(current).unwrap();
    let a = valve.connected.as_slice().iter()
        .map(|neigh| {
            explore(neigh, map,
                    time_left - 1,
                    score,
                    opened.clone(), mem)
        }).max().unwrap_or(score);
    let b = if !opened.contains(current) {
        let reward = valve.rate * (time_left - 1) as u16;
        let mut opened = opened.clone();
        opened.insert(current);
        explore(current, map, time_left.checked_sub(1).unwrap_or(0), score + reward as u32, opened, mem)
    } else { score };
    let score = cmp::max(a, b);
    mem.insert(format!("{}:{}:{:?}", current, time_left, opened), score);
    score
}

fn task01(input: Vec<Valve>) -> u32 {
    let opened = input.iter().filter(|valve| valve.rate == 0)
        .map(|valve| valve.id).collect::<HashSet<&str>>();
    let map = input.into_iter()
        .map(|valve| (valve.id, valve))
        .collect::<HashMap<&str, Valve>>();
    explore("AA", &map, 30, 0, opened, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day16.in")
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        println!("task01: {}", task01(valves));

        Ok(())
    }
}