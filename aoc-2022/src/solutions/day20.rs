use std::collections::HashMap;
use futures::SinkExt;
use crate::parser::parse;

type Item = i64;

trait Idx {
    fn idx(&self, offset: Item) -> usize;
    fn get_val_idx(&self, idx: Item) -> Item;
}

impl Idx for Vec<(Option<usize>, Item)> {
    fn idx(&self, offset: Item) -> usize {
        offset.rem_euclid(self.len() as Item - 1) as usize
    }

    fn get_val_idx(&self, idx: Item) -> Item {
        self.get(self.idx(idx))
            .map(|(_, val)| *val).unwrap()
    }
}


impl Idx for Vec<Item> {
    fn idx(&self, offset: Item) -> usize {
        let len = self.len() as Item;
        ((len + offset - 1).rem_euclid(len)) as usize + 1
    }

    fn get_val_idx(&self, idx: Item) -> Item {
        self.get(self.idx(idx))
            .map(|val| *val).unwrap()
    }
}

fn unwrap(input: &Vec<(Option<usize>, Item)>) -> Vec<Item> {
    input.into_iter()
        .map(|(_, val)| *val).collect::<Vec<Item>>()
}

fn wrap(input: &Vec<Item>) -> Vec<(Option<usize>, Item)> {
    input.into_iter()
        .enumerate()
        .map(|(idx, val)| (Some(idx), *val)).collect::<Vec<_>>()
}

fn mix(org_input: &Vec<(Option<usize>, Item)>, changed_input: Vec<(Option<usize>, Item)>) -> Vec<(Option<usize>, Item)> {
    let mut changed_input = changed_input.clone();
    for (org_idx, item) in org_input {
        if item == &0 {
            continue;
        }
        let new_idx = changed_input.iter()
            .position(|(idx, _)| idx == org_idx).unwrap();
        let new_idx = changed_input.idx(item + new_idx as Item);
        changed_input = changed_input.clone().into_iter()
            .filter(|(idx, _)| idx != org_idx).collect();
        changed_input.insert(new_idx, (*org_idx, *item));
    }

    changed_input
}

fn summarise(changed_input: &Vec<(Option<usize>, Item)>) -> Item {
    let changed_input = unwrap(&changed_input);
    let offset = changed_input.iter().position(|val| *val == 0).unwrap() as Item;
    vec![1000 + offset, 2000 + offset, 3000 + offset].into_iter()
        .map(|idx| changed_input.get_val_idx(idx))
        .sum()
}

fn task01(input: Vec<Item>) -> Item {
    let org_input = wrap(&input);
    let changed_input = mix(&org_input, org_input.clone());
    summarise(&changed_input)
}

fn task02(input: Vec<Item>) -> Item {
    let input = input.into_iter().map(|val| val * 811589153).collect::<Vec<Item>>();
    let org_input = wrap(&input);
    let mut changed_input = org_input.clone();

    for _ in 0..10 {
        changed_input = mix(&org_input, changed_input);
    }
    summarise(&changed_input)
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Item>, String> {
    input.into_iter()
        .map(|line| line.parse::<Item>().map_err(|err| err.to_string()))
        .collect()
}

fn input_data() -> Vec<String> {
    parse("resources/day20.in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let expected = vec![-2, 1, 2, -3, 4, 0, 3];
        let input = wrap(&input);
        assert_eq!(unwrap(&mix(&input, input.clone())), expected);
    }

    #[test]
    fn test_mix2() {
        let input = vec![8, 2, 32, -41, 6, 29, -4, 6, -8, 8, -3, -8, 3, -5, 0, -1, 2, 1, 10, -9];
        let expected = vec![-5, -3, 2, 8, 6, 6, 29, 32, 10, 3, -9, 8, 0, -1, -8, -41, -8, 2, -4, 1];
        let input = wrap(&input);
        assert_eq!(unwrap(&mix(&input, input.clone())), expected);
    }

    #[test]
    fn test_edge_case() {
        let input = vec![0, -1, -1, 1];
        let expected = vec![-1, 1, -1, 0];
        let input = wrap(&input);
        assert_eq!(unwrap(&mix(&input, input.clone())), expected);
    }

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