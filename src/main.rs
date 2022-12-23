use std::env;
use std::collections::HashMap;

mod day;
use day::DayFn;
mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(0).unwrap().as_str();
    let mut register = HashMap::<String, DayFn>::new();

    register.insert("01".to_string(), days::day01::day_01);
    register.insert("01b".to_string(), days::day01::day_01_b);
    register.insert("02".to_string(), days::day02::day_02);
    register.insert("02b".to_string(), days::day02::day_02_b);

    if let Some(day) = args.get(1) {
        if let Some(day_fn) = register.get(day) {
            let result = day_fn();

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

    println!("Usage: {command} <NN>");
    std::process::exit(1);
}
