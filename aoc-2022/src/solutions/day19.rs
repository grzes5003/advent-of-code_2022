use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops;
use std::ops::Deref;
use std::process::Output;
use fancy_regex::{CaptureMatches, Captures, Regex};
use crate::parser::parse;


type Size = u32;

trait Get {
    fn get(&self) -> Size;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Resource {
    Ore(Size),
    Clay(Size),
    Obsidian(Size),
    Geode(Size),
}

impl Get for Resource {
    fn get(&self) -> Size {
        match self {
            Resource::Ore(size) => *size,
            Resource::Clay(size) => *size,
            Resource::Obsidian(size) => *size,
            Resource::Geode(size) => *size,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Robots {
    Ore(Size),
    Clay(Size),
    Obsidian(Size),
    Geode(Size),
}

impl Get for Robots {
    fn get(&self) -> Size {
        match self {
            Robots::Ore(size) => *size,
            Robots::Clay(size) => *size,
            Robots::Obsidian(size) => *size,
            Robots::Geode(size) => *size,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: u8,
    ore: Size,
    clay: Size,
    obsidian: (Size, Size),
    geode: (Size, Size),
}

impl Blueprint {
    fn remove_resources(&self, robot: &Robots, resources: &[Resource; 4]) -> [Resource; 4] {
        match robot {
            Robots::Ore(_) =>
                [Resource::Ore(resources[0].get() - self.ore), resources[1].clone(), resources[2].clone(), resources[3].clone()],
            Robots::Clay(_) =>
                [Resource::Ore(resources[0].get() - self.clay), resources[1].clone(), resources[2].clone(), resources[3].clone()],
            Robots::Obsidian(_) =>
                [Resource::Ore(resources[0].get() - self.obsidian.0), Resource::Clay(resources[1].get() - self.obsidian.1), resources[2].clone(), resources[3].clone()],
            Robots::Geode(_) =>
                [Resource::Ore(resources[0].get() - self.geode.0), resources[1].clone(), Resource::Obsidian(resources[2].get() - self.geode.1), resources[3].clone()],
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    time_remaining: Size,
    robots: [Robots; 4],
    resource: [Resource; 4],
    blueprint: Blueprint,
    idled: Option<Vec<Robots>>,
}

impl State {
    fn new(time: Size, blueprint: &Blueprint) -> Self {
        Self {
            time_remaining: time,
            robots: [Robots::Ore(1), Robots::Clay(0), Robots::Obsidian(0), Robots::Geode(0)],
            resource: [Resource::Ore(0), Resource::Clay(0), Resource::Obsidian(0), Resource::Geode(0)],
            blueprint: blueprint.clone(),
            idled: None
        }
    }
}

impl State {
    fn run(&self, cache: &mut HashMap<&Self, Self>, best_score: Size) -> Size {
        if self.time_remaining == 1 {
            return self.robots[3].get() + self.resource[3].get();
        }

        if self.best_possible(&self.robots[3]) < best_score {
            return 0;
        }

        let available_robots = self.robots.iter()
            .filter(|robot| self.can_build(robot)).collect::<Vec<_>>();

        let new_state = Self {
            time_remaining: self.time_remaining - 1,
            robots: self.robots.clone(),
            resource: self.mine(),
            blueprint: self.blueprint.clone(),
            idled: None,
        };

        if self.can_build(&self.robots[3]) {
            let new_state = new_state.build_robot(&self.robots[3]);
            return new_state.run(cache, best_score);
        }

        let best_score = available_robots.iter().map(|robot| {
            if let Some(idled) = &self.idled {
                if idled.contains(robot) {
                    return 0;
                }
            }
            new_state.clone()
                .build_robot(robot)
                .run(cache, best_score)
        }).max().unwrap_or(best_score);

        std::cmp::max(best_score, new_state.set_idled(available_robots).run(cache, best_score))
    }

    fn best_possible(&self, robot: &Robots) -> Size {
        let t = self.time_remaining;
        let t_avg = t * (t - 1) / 2;
        (match robot {
            Robots::Ore(num) => self.resource[0].get() + num * t,
            Robots::Clay(num) => self.resource[1].get() + num * t,
            Robots::Obsidian(num) => self.resource[2].get() + num * t,
            Robots::Geode(num) => self.resource[3].get() + num * t,
        }) + t_avg
    }

    fn mine(&self) -> [Resource; 4] {
        [
            Resource::Ore(self.resource[0].get() + self.robots[0].get()),
            Resource::Clay(self.resource[1].get() + self.robots[1].get()),
            Resource::Obsidian(self.resource[2].get() + self.robots[2].get()),
            Resource::Geode(self.resource[3].get() + self.robots[3].get()),
        ]
    }

    fn can_build(&self, robot: &Robots) -> bool {
        let res = match robot {
            Robots::Ore(_) => self.blueprint.ore <= self.resource[0].get(),
            Robots::Clay(_) => self.blueprint.clay <= self.resource[0].get(),
            Robots::Obsidian(_) => self.blueprint.obsidian.0 <= self.resource[0].get()
                && self.blueprint.obsidian.1 <= self.resource[1].get(),
            Robots::Geode(_) =>
                self.blueprint.geode.0 <= self.resource[0].get()
                && self.blueprint.geode.1 <= self.resource[2].get(),
        };
        res
    }

    fn build_robot(self, robot: &Robots) -> Self {
        let robots = match robot {
            Robots::Ore(_) =>
                [Robots::Ore(1 + self.robots[0].get()), self.robots[1].clone(), self.robots[2].clone(), self.robots[3].clone()],
            Robots::Clay(_) =>
                [self.robots[0].clone(), Robots::Clay(1 + self.robots[1].get()), self.robots[2].clone(), self.robots[3].clone()],
            Robots::Obsidian(_) =>
                [self.robots[0].clone(), self.robots[1].clone(), Robots::Obsidian(1 + self.robots[2].get()), self.robots[3].clone()],
            Robots::Geode(_) =>
                [self.robots[0].clone(), self.robots[1].clone(), self.robots[2].clone(), Robots::Geode(1 + self.robots[3].get())]
        };

        Self {
            time_remaining: self.time_remaining,
            robots,
            resource: self.blueprint.remove_resources(robot, &self.resource),
            blueprint: self.blueprint,
            idled: self.idled,
        }
    }

    fn set_idled(&self, idled: Vec<&Robots>) -> Self {
        Self {
            time_remaining: self.time_remaining,
            robots: self.robots.clone(),
            resource: self.resource.clone(),
            blueprint: self.blueprint.clone(),
            idled: Some(idled.into_iter().map(|robot| robot.clone()).collect()),
        }
    }
}

fn task01(input: Vec<Blueprint>) -> Size {
    let mut cache = HashMap::new();
    input.into_iter().map(|blueprint| {
        let state = State::new(24, &blueprint);
        let res = state.run(&mut cache, 0);
        blueprint.id as Size * res
    }).sum()
}

fn task02(input: Vec<Blueprint>) -> Size {
    let mut cache = HashMap::new();
    input.into_iter().take(3).map(|blueprint| {
        let state = State::new(32, &blueprint);
        state.run(&mut cache, 0)
    }).fold(1, |acc, x| acc * x)
}

fn parse_cap<T>(captures: &mut CaptureMatches) -> Result<T, String>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display {
    captures.next()
        .ok_or("Not found".to_string())?.map_err(|e| e.to_string())?
        .get(1).ok_or("Not found".to_string())?
        .as_str().parse::<T>()
        .map_err(|err| err.to_string())
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Blueprint>, String> {
    let re = Regex::new(r"(\d+)(?:\s*(?:ore|clay|obsidian|geode))?")
        .map_err(|e| e.to_string())?;
    input.into_iter()
        .map(|line| {
            let mut res = re.captures_iter(&line);
            let id = parse_cap(&mut res)?;
            let ore = parse_cap(&mut res)?;
            let clay = parse_cap(&mut res)?;
            let obsidian = (parse_cap(&mut res)?, parse_cap(&mut res)?);
            let geode = (parse_cap(&mut res)?, parse_cap(&mut res)?);
            Ok(Blueprint { id, ore, clay, obsidian, geode })
        }).collect::<Result<Vec<Blueprint>, String>>()
}

fn input_data() -> Vec<String> {
    parse("resources/day19.in")
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