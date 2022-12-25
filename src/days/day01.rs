use crate::utils::{*};

fn calculate_highest(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    let groups = SplitIter::new(lines, |s| s == "");
    let totals = groups.map(|g| g.iter().map(|s| s.trim().parse::<i32>().unwrap()).sum());
    let high = highest(totals).unwrap();
    high
}

fn calculate_sum_of_top_3(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    let groups = SplitIter::new(Box::new(lines), |s| s == "");
    let totals = groups.map(|g| g.iter().map(|s| s.trim().parse::<i32>().unwrap()).sum());
    let top3 = top_n_values(totals, 3);
    let combined = top3.into_iter().sum();
    combined
}

// Find the highest sum of values separated by empty line
pub fn day_01() -> Result<i32, &'static str> {
    let lines = read_lines("inputs/day01.txt")
        .unwrap()
        .map(|l| l.unwrap());
    Ok(calculate_highest(Box::new(lines)))
}

// Find the sum of the top three highest sum of values, separated by empty line
pub fn day_01_b() -> Result<i32, &'static str> {
    let lines = read_lines("inputs/day01.txt")
        .unwrap()
        .map(|l| l.unwrap());
    Ok(calculate_sum_of_top_3(Box::new(lines)))
}

#[cfg(test)]
mod test {
    #[test]
    fn example_a() {
        let lines = [
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
        ].map(|s| s.to_string());

        let actual = super::calculate_highest(Box::new(lines.into_iter()));
        let expected = 24000;

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_b() {
        let lines = [
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
        ].map(|s| s.to_string());

        let actual = super::calculate_sum_of_top_3(Box::new(lines.into_iter()));
        let expected = 45000;

        assert_eq!(actual, expected);
    }
}