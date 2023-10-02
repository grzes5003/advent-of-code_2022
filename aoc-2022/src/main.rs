#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod parser;
mod solutions;
use solutions::day15::{task02, parse_input};
use crate::parser::parse;
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    let (sonars, _) =
        parse_input(parse("resources/day15.in"))?;
    println!("task02: {:?}", task02(sonars).await);

    Ok(())
}