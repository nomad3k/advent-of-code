use std::ops::RangeInclusive;
use crate::utils::read_lines;

fn split(line: String) -> Option<(String, String)> {
    let mut iter = line.trim().split(",");
    let first = iter.next()?.to_string();
    let second = iter.next()?.to_string();
    Some((first, second))
}

fn parse(s: String) -> RangeInclusive<i32> {
    let mut iter = s.trim().split("-");
    let start = iter.next().unwrap().parse::<i32>().unwrap();
    let end = iter.next().unwrap().parse::<i32>().unwrap();

    RangeInclusive::new(start, end)
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    b.start() <= a.end() && b.end() >= a.start()
}

fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && b.end() <= a.end()
}

fn redundant(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    contains(a, b) || contains(b, a)
}

fn count_redundant_pairs<T>(lines: T) -> usize
    where T: IntoIterator<Item=String>
{
    lines.into_iter()
         .map(|line| split(line).unwrap())
         .map(|(a,b)| (parse(a), parse(b)))
         .filter(|(a,b)| redundant(&a, &b))
         .count()
}

fn count_overlapping_pairs<T>(lines: T) -> usize
    where T: IntoIterator<Item=String>
{
    lines.into_iter()
         .map(|line| split(line).unwrap())
         .map(|(a,b)| (parse(a), parse(b)))
         .filter(|(a,b)| overlaps(&a, &b))
         .count()
}

pub fn day_04() -> Result<i32, &'static str>
{
    let lines = read_lines("inputs/day04.txt").unwrap().map(|s| s.unwrap().to_string());
    Ok(count_redundant_pairs(lines) as i32)
}

pub fn day_04_b() -> Result<i32, &'static str>
{
    let lines = read_lines("inputs/day04.txt").unwrap().map(|s| s.unwrap().to_string());
    Ok(count_overlapping_pairs(lines) as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn overlaps() {
        assert_eq!(super::overlaps(&(1..=5), &(6..=10)), false);
        assert_eq!(super::overlaps(&(6..=10), &(1..=5)), false);
        assert_eq!(super::overlaps(&(1..=5), &(5..=10)), true);
        assert_eq!(super::overlaps(&(5..=10), &(1..=5)), true);
        assert_eq!(super::overlaps(&(1..=10), &(4..=6)), true);
        assert_eq!(super::overlaps(&(4..=6), &(1..=10)), true);
    }

    #[test]
    fn contains() {
        assert_eq!(super::contains(&(2..=5), &(2..=5)), true);
        assert_eq!(super::contains(&(2..=5), &(3..=4)), true);
        assert_eq!(super::contains(&(2..=5), &(1..=6)), false);
        assert_eq!(super::contains(&(2..=5), &(1..=5)), false);
        assert_eq!(super::contains(&(2..=5), &(2..=6)), false);
        assert_eq!(super::contains(&(1..=5), &(2..=5)), true);
        assert_eq!(super::contains(&(2..=6), &(2..=5)), true);
        assert_eq!(super::contains(&(1..=5), &(2..=6)), false);
        assert_eq!(super::contains(&(2..=6), &(1..=5)), false);
        assert_eq!(super::contains(&(1..=5), &(5..=10)), false);
    }

    #[test]
    fn redundant() {
        assert_eq!(super::redundant(&(2..=5), &(2..=5)), true);
        assert_eq!(super::redundant(&(2..=5), &(3..=4)), true);
        assert_eq!(super::redundant(&(2..=5), &(1..=6)), true);
        assert_eq!(super::redundant(&(2..=5), &(1..=5)), true);
        assert_eq!(super::redundant(&(2..=5), &(2..=6)), true);
        assert_eq!(super::redundant(&(1..=5), &(2..=5)), true);
        assert_eq!(super::redundant(&(2..=6), &(2..=5)), true);
        assert_eq!(super::redundant(&(1..=5), &(2..=6)), false);
        assert_eq!(super::redundant(&(2..=6), &(1..=5)), false);
        assert_eq!(super::redundant(&(1..=5), &(5..=10)), false);
    }

    #[test]
    fn example_a() {
        let lines = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let actual = count_redundant_pairs(lines);
        let expected = 2;

        assert_eq!(actual, expected);
    }

    fn example_b() {
        let lines = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let actual = count_overlapping_pairs(lines);
        let expected = 4;

        assert_eq!(actual, expected);
    }
}