use std::collections::HashMap;
use crate::parser::parse;

type Num = i64;

#[derive(Debug, Clone)]
enum Operator {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Operator {
    fn num(&self, index: &HashMap<String, Monkey>) -> Result<Num, String> {
        match self {
            Operator::Add(a, b) => {
                let a = index.get(a).ok_or("a not found")?.get_num().ok_or("a not num")?;
                let b = index.get(b).ok_or("b not found")?.get_num().ok_or("b not num")?;
                Ok(a + b)
            },
            Operator::Sub(a, b) => {
                let a = index.get(a).ok_or("a not found")?.get_num().ok_or("a not num")?;
                let b = index.get(b).ok_or("b not found")?.get_num().ok_or("b not num")?;
                Ok(a - b)
            },
            Operator::Mul(a, b) => {
                let a = index.get(a).ok_or("a not found")?.get_num().ok_or("a not num")?;
                let b = index.get(b).ok_or("b not found")?.get_num().ok_or("b not num")?;
                Ok(a * b)
            },
            Operator::Div(a, b) => {
                let a = index.get(a).ok_or("a not found")?.get_num().ok_or("a not num")?;
                let b = index.get(b).ok_or("b not found")?.get_num().ok_or("b not num")?;
                Ok(a / b)
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    _type: Type,
}

#[derive(Debug, Clone)]
enum Type {
    OP(Operator),
    Num(Num),
}

impl Monkey {
    fn children(&self) -> Vec<String> {
        match &self._type {
            Type::OP(op) => {
                match op {
                    Operator::Add(a, b) => vec![a.clone(), b.clone()],
                    Operator::Sub(a, b) => vec![a.clone(), b.clone()],
                    Operator::Mul(a, b) => vec![a.clone(), b.clone()],
                    Operator::Div(a, b) => vec![a.clone(), b.clone()],
                }
            },
            Type::Num(_) => vec![],
        }
    }

    fn resolve(&self, index: &HashMap<String, Monkey>) -> Result<Self, String> {
        match self._type.clone() {
            Type::OP(op) => {
                let num = op.num(index)?;
                Ok(Monkey {
                    name: self.name.clone(),
                    _type: Type::Num(num),
                })
            },
            Type::Num(_) => Ok(self.clone()),
        }
    }

    fn get_num(&self) -> Option<Num> {
        match self._type.clone() {
            Type::OP(_) => None,
            Type::Num(num) => Some(num as Num),
        }
    }
}

fn bfs(tree: &HashMap<String, Monkey>, root: String) -> Vec<String> {
    let mut queue = vec![root];
    let mut visited = vec![];

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        visited.push(node.clone());
        for child in tree.get(&node).unwrap().children() {
            if !visited.contains(&child) {
                queue.push(child);
            }
        }
    }

    visited.into_iter()
        .rev()
        .collect()
}

fn resolve(input: &Vec<Monkey>, root: String) -> Result<Num, String> {
    let mut index = input.iter()
        .map(|m| (m.name.clone(), m.to_owned()))
        .collect::<HashMap<String, Monkey>>();

    for name in bfs(&index, root.clone()) {
        let idx_clone = index.clone();
        let monkey = index.get_mut(&name).ok_or("Monkey not found".to_string())?;
        *monkey = monkey.resolve(&idx_clone)?;
        // println!("{}: {:?}", name, monkey);
    }
    index.get(root.as_str()).ok_or("Root not found".to_string())?
        .get_num().ok_or("Root not num".to_string())
}

fn task01(input: Vec<Monkey>) -> Result<Num, String> {
    resolve(&input, "root".to_string())
}

fn replace_monkey(input: &Vec<Monkey>, monkey: Monkey) -> Vec<Monkey> {
    input.iter()
        .map(|m| {
            if m.name == monkey.name {
                monkey.clone()
            } else {
                m.to_owned()
            }
        }).collect()
}

fn task02(input: Vec<Monkey>, root1: String, root2: String) -> Option<Num> {
    let mut num1 = Num::MIN;
    let mut num2 = Num::MAX;
    let mut counter: Num = 0;

    let mut input = input.clone();
    while num1 != num2 {
        if counter % 10000 == 0 {
            println!("counter: {}", counter);
        }
        if counter == Num::MAX {
            return None;
        }
        input = replace_monkey(&input, Monkey {
            name: "humn".to_string(),
            _type: Type::Num(counter),
        });
        num1 = resolve(&input, root1.clone()).unwrap();
        num2 = resolve(&input, root2.clone()).unwrap();
        counter += 1;
    }
    Some(counter - 1)
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Monkey>, String> {
    input.into_iter()
        .map(|line| {
            let mut parts = line.split(":");
            let name = parts.next().ok_or("name not found")?;
            let body = parts.next().ok_or("body not found")?;

            let _type = match body.trim().split_whitespace().collect::<Vec<&str>>() {
                item if item.len() == 1 => Type::Num(item[0].parse::<Num>().map_err(|e| format!("num: {}", e))?),
                item if item.len() == 3 => {
                    let op = match item[1] {
                        "+" => Operator::Add(item[0].to_string(), item[2].to_string()),
                        "-" => Operator::Sub(item[0].to_string(), item[2].to_string()),
                        "*" => Operator::Mul(item[0].to_string(), item[2].to_string()),
                        "/" => Operator::Div(item[0].to_string(), item[2].to_string()),
                        _ => return Err(format!("unknown operator: {}", item[1])),
                    };
                    Type::OP(op)
                },
                _ => return Err(format!("unknown body: {}", body)),
            };
            
            Ok(Monkey {
                name: name.to_string(),
                _type,
            })
        }).collect()
}

fn input_data() -> Vec<String> {
    parse("resources/day21.in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        println!("val: {:?}", valves);
        println!("task01: {}", task01(valves)?);

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data)?;
        let result = task02(valves, "bsbd".to_string(), "fcgj".to_string())
            .ok_or("task02 error".to_string())?;
        println!("task02: {}", result);

        Ok(())
    }
}
