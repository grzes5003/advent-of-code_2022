use std::collections::VecDeque;
use std::f64;
use crate::parser::parse;

type Num = i64;
struct Snafu {
    _dec: Option<Num>,
    _snafu: String
}

impl Snafu {
    fn dec(&mut self) -> Num {
        match self._dec {
            None => self.decode(),
            Some(val) => val
        }
    }

    fn snafu(&self) -> &String {
        &self._snafu
    }

    fn decode(&mut self) -> Num {
        let mut buff = self._snafu.chars().rev().enumerate();
        let mut num = 0;
        while let Some((power, ch)) = buff.next() {
            let n = match ch {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unexpected char: {}", ch)
            };
            num += n * 5i64.pow(power as u32);
        };
        self._dec = Some(num);
        num
    }

    fn from_dec(num: Num) -> Self {
        let mut buff = VecDeque::new();
        let mut num = num;
        while num != 0 {
            let ch = match (num + 2) % 5 {
                0 => '=',
                1 => '-',
                2 => '0',
                3 => '1',
                4 => '2',
                n => panic!("Unexpected num: {}", n)
            };
            buff.push_front(ch);
            num = (num + 2) / 5;
        }
        Self {
            _dec: Some(num),
            _snafu: buff.into_iter().collect()
        }
    }
}

fn task01(snafus: Vec<Snafu>) -> String {
    let dec = snafus.into_iter()
        .map(|mut snafu| snafu.dec())
        .sum();
    Snafu::from_dec(dec)._snafu
}

fn parse_input(input: &Vec<String>) -> Vec<Snafu> {
    input.into_iter()
        .map(|line| Snafu {
            _dec: None,
            _snafu: line.to_owned()
        }).collect()
}

fn input_data() -> Vec<String> {
    parse("resources/day25.in")
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use super::*;

    #[test]
    fn decode_test() {
        let mut snafu = Snafu {
            _dec: None,
            _snafu: "0".to_owned()
        };
        assert_eq!(snafu.dec(), 0);
        let mut snafu = Snafu {
            _dec: None,
            _snafu: "1=11-2".to_owned()
        };
        assert_eq!(snafu.dec(), 2022);
    }

    #[test]
    fn encode_test() {
        assert_eq!(Snafu::from_dec(6)._snafu, "11".to_owned());
        assert_eq!(Snafu::from_dec(10)._snafu, "20".to_owned());
        assert_eq!(Snafu::from_dec(906)._snafu, "12111".to_owned());
        assert_eq!(Snafu::from_dec(1257)._snafu, "20012".to_owned());
    }

    #[test]
    fn task01_test() -> Result<(), String> {
        let input_data = input_data();
        let valves = parse_input(&input_data);
        println!("task01: {:?}", task01(valves));

        Ok(())
    }
}