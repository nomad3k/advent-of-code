use std::env;
use std::collections::HashMap;

mod day;
use day::DayFn;
mod days;
mod utils;
use utils::*;

fn life() -> Result<i32, &'static str> { 
    Ok(42)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(0).unwrap();

    let mut register = HashMap::<String, DayFn>::new();
    register.insert("meaning_of_life".to_string(), life);
    register.insert("01".to_string(), days::day01::day_01);
    register.insert("01b".to_string(), days::day01::day_01_b);
    register.insert("02".to_string(), days::day02::day_02);
    register.insert("02b".to_string(), days::day02::day_02_b);
    register.insert("03".to_string(), days::day03::day_03);
    register.insert("03b".to_string(), days::day03::day_03_b);

    if let Some(day) = args.get(1) {
        if let Some(day_fn) = register.get(day) {
            match day_fn() {
                Err(message) => {
                    println!("{message}");
                    std::process::exit(2);
                },
                Ok(result) => {
                    let answer = format!(" The answer is {result} ");
                    let split = "-".repeat(answer.len());
        
                    println!();
                    println!("{}", split);
                    println!("{}", answer);
                    println!("{}", split);
                    println!();
        
                    std::process::exit(0);
                }
            }
        }
    }

    println!();
    println!("Usage: {command} <option>");
    let options = register.keys().map(|s| s.to_string()).sorted().collect::<Vec<String>>().join(", ");
    println!("  option: {options}");
    println!();
    std::process::exit(1);
}
