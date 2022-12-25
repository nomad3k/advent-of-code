use crate::utils::*;

fn priority(c: char) -> Result<i32, &'static str> {
    let b = c as u8;
    match b {
        97..=122 => Ok(b as i32 - 96),
        65..=90 => Ok(b as i32 - 64 + 26),
        _ => Err("Invalid input")
    }   
}

fn comparments(line: String) -> Result<(String, String), &'static str> {
    let len = line.len();
    if len % 2 != 0 {
        Err("Invalid input")
    } else {
        let (first, second) = line.split_at(len / 2);
        Ok((first.to_string(), second.to_string()))
    }
}

fn first_common_item(first: String, second: String) -> Option<char> {
    let first: Vec<char> = first.chars().sorted().collect();
    let second: Vec<char> = second.chars().sorted().collect();
    let mut fi = 0; 
    let mut si = 0;

    while fi < first.len() && si < second.len() {
        let f = *first.get(fi)?;
        let s = *second.get(si)?;
        if f == s {
            return Some(f);
        } else if s > f {
            fi = fi + 1;
        } else {
            si = si + 1;
        }
    }
    None
}

fn calculate_sum_of_common_priorities(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    lines.map(|line| comparments(line).unwrap())
         .map(|(f, s)| first_common_item(f, s).unwrap())
         .map(|i| priority(i).unwrap())
         .sum()
}

fn common_item<T>(lines: T) -> Option<char>
    where T: IntoIterator<Item=String>
{
    // de-duplicate each line and sort so the shortest is first
    let mut x = lines.into_iter()
                 .map(|x|->String {x.chars().sorted().deduped().collect()})
                 .sorted_by(|a,b| a.len().cmp(&b.len()))
                 .into_iter(); 
    let head: String = x.next().unwrap();
    let tail:  Vec<String> = x.collect();
    'head: for ch in head.chars() {
        for line in &tail {
            if !line.contains(ch) {
                continue 'head;
            }
        }
        return Some(ch);
    }
    None
}

fn calculate_sum_of_3_line_common_items(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    lines.split_nth(3)
        .map(|group| common_item(group).unwrap())
        .map(|item| priority(item).unwrap())
        .sum()
}

pub fn day_03() -> Result<i32, &'static str> {
    let lines = read_lines("inputs/day03.txt").unwrap();
    Ok(calculate_sum_of_common_priorities(Box::new(lines.map(|line| line.unwrap()))))
}

pub fn day_03_b() -> Result<i32, &'static str> {
    let lines = read_lines("inputs/day03.txt").unwrap();
    Ok(calculate_sum_of_3_line_common_items(Box::new(lines.map(|line| line.unwrap()))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_03_a() {
        assert_eq!(super::first_common_item("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()), Some('p'));
        assert_eq!(super::first_common_item("jqHRNqRjqzjGDLGL".to_string(), "rsFMfFZSrLrFZsSL".to_string()), Some('L'));
        assert_eq!(super::first_common_item("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()), Some('P'));
        assert_eq!(super::first_common_item("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()), Some('v'));
        assert_eq!(super::first_common_item("ttgJtRGJ".to_string(), "QctTZtZT".to_string()), Some('t'));
        assert_eq!(super::first_common_item("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()), Some('s'));

        assert_eq!(super::priority('p'), Ok(16));
        assert_eq!(super::priority('L'), Ok(38));
        assert_eq!(super::priority('P'), Ok(42));
        assert_eq!(super::priority('v'), Ok(22));
        assert_eq!(super::priority('t'), Ok(20));
        assert_eq!(super::priority('s'), Ok(19));

        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ].into_iter().map(|s| s.to_string());
        
        let actual = calculate_sum_of_common_priorities(Box::new(lines));
        let expected = 157;

        assert_eq!(actual, expected);
    }

    #[test]
    fn example_03_b() {
        let first = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ].into_iter().map(|s| s.to_string());
        let second = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].into_iter().map(|s| s.to_string());

        assert_eq!(super::priority('r').unwrap() + super::priority('Z').unwrap(), 70);
        assert_eq!(super::common_item(Box::new(first)), Some('r'));
        assert_eq!(super::common_item(Box::new(second)), Some('Z'));
    }

    #[test]
    fn xx() {
        assert_eq!('a' as u8, 97);
        assert_eq!('A' as u8, 65);
    }

    #[test]
    fn priority() {
        assert_eq!(super::priority('a'), Ok(1));
        assert_eq!(super::priority('z'), Ok(26));
        assert_eq!(super::priority('A'), Ok(27));
        assert_eq!(super::priority('Z'), Ok(52));
        assert_eq!(super::priority('~'), Err("Invalid input"));
    }

    #[test]
    fn compartments() {
        assert_eq!(super::comparments("abcdef".to_string()), Ok(("abc".to_string(), "def".to_string())));
        assert_eq!(super::comparments("xy".to_string()), Ok(("x".to_string(), "y".to_string())));
        assert_eq!(super::comparments("x".to_string()), Err("Invalid input"));
    }

    #[test]
    fn first_common_item() {
        assert_eq!(super::first_common_item("abcd".to_string(), "ABCD".to_string()), None);
    }
}