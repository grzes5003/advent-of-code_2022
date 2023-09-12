use std::cmp::Ordering;

type Pair = (Packet, Packet);
type Val = u16;

#[derive(Debug, Clone)]
enum Packet {
    Empty,
    Num(Val),
    Packet(Vec<Box<Packet>>)
}

impl Packet {

    fn new(raw: String) -> Result<Packet, String> {
        if !raw.starts_with('[') || !raw.ends_with(']') {
            return Err(format!("Invalid packet: {}", raw));
        }

        let raw = &raw[1..raw.len() - 1];
        if raw.is_empty() {
            return Ok(Packet::Empty);
        }

        if raw.chars().all(|c| c.is_digit(10)) {
            return Ok(Packet::Num(raw.parse::<Val>().unwrap()));
        }

        let tokens = Packet::tokenizer(raw.to_owned()).into_iter()
            .map(|token| match token {
                t if t.starts_with('[') => Packet::new(t.to_owned()),
                t if t.chars().all(|c| c.is_digit(10)) => Ok(Packet::Num(t.parse::<Val>().unwrap())),
                _ => Err(format!("Invalid token: {}", token))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|tokens| tokens.into_iter().map(|t| Box::new(t)).collect::<Vec<_>>() )
            .map_err(|e| format!("Invalid packet: {} in {}", e, raw));

        Ok(Packet::Packet(tokens?))
    }

    fn wrap(&self) -> Packet {
        match self {
            Packet::Packet(a) =>
                Packet::Packet(a.iter().map(|p| Box::new(p.wrap())).collect()),
            Packet::Num(num) => Packet::Packet(vec![Box::new(Packet::Num(*num))]),
            Packet::Empty => Packet::Packet(vec![])
        }
    }

    fn tokenizer(input: String) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut input_iter = input.chars();
        let mut stack = Vec::new();

        let mut opening = 0; let mut closing = 0;

        while let Some(c) = input_iter.next() {
            match c {
                ',' => {
                    if opening == closing {
                        tokens.push(stack.iter().collect::<String>());
                        stack.clear();
                    } else {
                        stack.push(c);
                    }
                },
                '[' => {
                    opening += 1;
                    stack.push(c);
                },
                ']' => {
                    closing += 1;
                    stack.push(c);
                },
                _ => stack.push(c)
            }
        }
        tokens.push(stack.iter().collect::<String>());
        tokens
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Empty, Packet::Empty) => Ordering::Equal,
            (Packet::Empty, _) => Ordering::Less,
            (_, Packet::Empty) => Ordering::Greater,
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (Packet::Num(_), Packet::Packet(_)) => self.wrap().cmp(other),
            (Packet::Packet(_), Packet::Num(_)) => self.cmp(&other.wrap()),
            (Packet::Packet(a), Packet::Packet(b)) => {
                let ordering = a.into_iter().zip(b.into_iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|&ord| ord != Ordering::Equal)
                    .unwrap_or(Ordering::Equal);
                match ordering {
                    Ordering::Equal => {
                        if a.len() < b.len() {
                            Ordering::Less
                        } else if a.len() > b.len() {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                    ord => ord
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Packet::Empty, Packet::Empty) => true,
            (Packet::Empty, _) => false,
            (_, Packet::Empty) => false,
            (Packet::Num(a), Packet::Num(b)) => a == b,
            (Packet::Num(_), Packet::Packet(_)) => self.clone().wrap() == *other,
            (Packet::Packet(_), Packet::Num(_)) => self == &other.wrap(),
            (Packet::Packet(a), Packet::Packet(b)) => {
                a.into_iter().zip(b.into_iter())
                    .all(|(a, b)| a == b)
            }
        }
    }
}

impl Eq for Packet {}

fn task01(input: Vec<Pair>) -> u16 {
    let res = input.into_iter().enumerate()
        .map(|(idx, (a, b))| {
            if a < b {
                idx as u16 + 1
            } else { 0 }
        }).collect::<Vec<_>>();
    res.into_iter().sum()
}

fn task02(input: Vec<Pair>) -> u16 {
    let mut res = input.into_iter()
        .chain(vec![
            (Packet::Num(2).wrap(), Packet::Num(6).wrap())
        ])
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();
    res.sort();

    let i = res.iter()
        .position(|p| p.eq(&Packet::Num(2).wrap())).unwrap() as u16 + 1;
    let j = res.iter()
        .position(|p| p.eq(&Packet::Num(6).wrap())).unwrap() as u16 + 1;
    i * j
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn parse_input(input: Vec<String>) -> Result<Vec<Pair>, String> {
        input.split(|line| line.is_empty())
            .map(|lines| {
                if let [body, tail] = lines {
                    let body_packet = Packet::new(body.to_owned());
                    let tail_packet = Packet::new(tail.to_owned());
                    tail_packet.and_then(|t_p|
                        body_packet.and_then(|b_p|
                            Ok((b_p, t_p))
                        ))
                } else {
                    panic!("Invalid input")
                }
            })
            .collect::<Result<Vec<Pair>, String>>()
    }

    fn input_data() -> Vec<String> {
        parse("resources/day13.in")
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_pairs = parse_input(input_data())?;
        let res = task01(input_pairs);
        println!("task01: {}", res);

        Ok(())
    }

    #[test]
    fn task02_test() -> Result<(), String> {
        let input_pairs = parse_input(input_data())?;
        let res = task02(input_pairs);
        println!("task02: {}", res);

        Ok(())
    }
}