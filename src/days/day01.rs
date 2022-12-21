use std::path::{Path};
use std::fs::{File};
use std::io::{self, BufReader, BufRead, Lines};
use crate::utils::{*};

fn read_lines(filename: impl AsRef<Path>) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

// Find the highest sum of values separated by empty line
pub fn day_01() -> i32 {
    let lines = read_lines("src/days/day01.txt")
        .unwrap()
        .map(|l| l.unwrap());
    
    let groups = SplitIter::new(Box::new(lines), |s| s == "");
    let totals = groups.map(|g| g.iter().map(|s| s.trim().parse::<i32>().unwrap()).fold(0,|a,b|a+b));
    let high = highest(totals).unwrap();

    high
}

// Find the sum of the top three highest sum of values, separated by empty line
pub fn day_01_b() -> i32 {
    let lines = read_lines("src/days/day01.txt")
        .unwrap()
        .map(|l| l.unwrap());

    let groups = SplitIter::new(Box::new(lines), |s| s == "");
    let totals = groups.map(|g| g.iter().map(|s| s.trim().parse::<i32>().unwrap()).fold(0,|a,b|a+b));
    let top3 = top_n_values(totals, 3);
    let combined = sum(top3.into_iter());

    combined
}