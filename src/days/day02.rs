use crate::utils::*;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Outcome {
    Win = 0,
    Lose = 1,
    Draw = 2
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Round(Shape, Shape);

fn what_it_beats(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

// TODO: Don't like the fact this relationship is defined twice.  Chance for errors.
fn what_beats_it(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn calculate_outcome(mine: Shape, theirs: Shape) -> Outcome {
    if theirs == mine {
        Outcome::Draw
    } else if mine == what_beats_it(theirs) {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn score_from_shape(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn score_from_outcome(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn score_round(mine: Shape, theirs: Shape) -> i32 {
    let outcome = calculate_outcome(mine.clone(), theirs);
    score_from_outcome(outcome.clone()) + score_from_shape(mine)
}

fn parse_their_shape(c: char) -> Result<Shape, String> {
    match c {
        'A' => Ok(Shape::Rock),
        'B' => Ok(Shape::Paper),
        'C' => Ok(Shape::Scissors),
        _ => Err(format!("Invalid Input '{c}'"))
    }
}

fn parse_my_shape(c: char) -> Result<Shape, String> {
    match c {
        'X' => Ok(Shape::Rock),
        'Y' => Ok(Shape::Paper),
        'Z' => Ok(Shape::Scissors),
        _ => Err(format!("Invalid Input '{c}'"))
    }
}

fn parse_outcome(c: char) -> Result<Outcome, &'static str> {
    match c {
        'X' => Ok(Outcome::Lose),
        'Y' => Ok(Outcome::Draw),
        'Z' => Ok(Outcome::Win),
        _ => Err("Invalid Input")
    }
}

fn chose_shape_to_produce_outcome(theirs: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Draw => theirs,
        Outcome::Win => what_beats_it(theirs),
        Outcome::Lose => what_it_beats(theirs),
    }
}

fn shapes_from_line(line: String) -> Option<Round> {
    let mut split = line.split(" ");
    let theirs = parse_their_shape(split.next()?.chars().next()?).unwrap();
    let mine = parse_my_shape(split.next()?.chars().next()?).unwrap();
    Some(Round(mine, theirs))
}

fn shapes_from_line_with_outcome(line: String) -> Option<Round> {
    let mut split = line.split(" ");
    let theirs = parse_their_shape(split.next()?.chars().next()?).unwrap();
    let needed = parse_outcome(split.next()?.chars().next()?).unwrap();
    let mine = chose_shape_to_produce_outcome(theirs.clone(), needed);
    Some(Round(mine, theirs))
}

fn calculate_total_score_from_shape(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    lines.map(|line| shapes_from_line(line).unwrap())
        .map(|round| { let Round(mine, theirs) = round; score_round(mine, theirs) } )
        .into_iter()
        .fold(0, |a,b| a +b)
}

fn calculate_total_score_from_outcome(lines: Box<dyn Iterator<Item=String>>) -> i32 {
    lines.map(|line| shapes_from_line_with_outcome(line).unwrap())
        .map(|round| { let Round(mine, theirs) = round; score_round(mine, theirs) } )
        .into_iter()
        .fold(0, |a,b| a +b)
}

pub fn day_02() -> i32 {
    let lines = read_lines("inputs/day02.txt").unwrap();
    calculate_total_score_from_shape(Box::new(lines.map(|line| line.unwrap())))
}

pub fn day_02_b() -> i32 {
    let lines = read_lines("inputs/day02.txt").unwrap();
    calculate_total_score_from_outcome(Box::new(lines.map(|line| line.unwrap())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shape_equality() {
        assert_eq!(Shape::Rock, Shape::Rock);
        assert_eq!(Shape::Paper, Shape::Paper);
        assert_eq!(Shape::Scissors, Shape::Scissors);
    }

    #[test]
    fn shape_inequality() {
        assert!(Shape::Rock != Shape::Paper);
        assert!(Shape::Rock != Shape::Scissors);
        assert!(Shape::Paper != Shape::Rock);
        assert!(Shape::Paper != Shape::Scissors);
        assert!(Shape::Scissors != Shape::Rock);
        assert!(Shape::Scissors != Shape::Paper);
    }

    #[test]
    fn what_it_beats() {
        assert_eq!(super::what_it_beats(Shape::Rock), Shape::Scissors);
        assert_eq!(super::what_it_beats(Shape::Paper), Shape::Rock);
        assert_eq!(super::what_it_beats(Shape::Scissors), Shape::Paper);
    }

    #[test]
    fn what_beats_it() {
        assert_eq!(super::what_beats_it(Shape::Rock), Shape::Paper);
        assert_eq!(super::what_beats_it(Shape::Paper), Shape::Scissors);
        assert_eq!(super::what_beats_it(Shape::Scissors), Shape::Rock);
    }

    #[test]
    fn outcome_equality() {
        assert_eq!(Outcome::Win, Outcome::Win);
        assert_eq!(Outcome::Lose, Outcome::Lose);
        assert_eq!(Outcome::Draw, Outcome::Draw);
        assert!(Outcome::Win != Outcome::Lose);
        assert!(Outcome::Win != Outcome::Draw);
        assert!(Outcome::Lose != Outcome::Win);
        assert!(Outcome::Lose != Outcome::Draw);
        assert!(Outcome::Draw != Outcome::Win);
        assert!(Outcome::Draw != Outcome::Lose);
    }

    #[test]
    fn outcome() {
        assert_eq!(super::calculate_outcome(Shape::Rock, Shape::Rock), Outcome::Draw);
        assert_eq!(super::calculate_outcome(Shape::Rock, Shape::Paper), Outcome::Lose);
        assert_eq!(super::calculate_outcome(Shape::Rock, Shape::Scissors), Outcome::Win);
        assert_eq!(super::calculate_outcome(Shape::Paper, Shape::Rock), Outcome::Win);
        assert_eq!(super::calculate_outcome(Shape::Paper, Shape::Paper), Outcome::Draw);
        assert_eq!(super::calculate_outcome(Shape::Paper, Shape::Scissors), Outcome::Lose);
        assert_eq!(super::calculate_outcome(Shape::Scissors, Shape::Rock), Outcome::Lose);
        assert_eq!(super::calculate_outcome(Shape::Scissors, Shape::Paper), Outcome::Win);
        assert_eq!(super::calculate_outcome(Shape::Scissors, Shape::Scissors), Outcome::Draw);
    }

    #[test]
    fn outcome_score() {
        assert_eq!(super::score_from_outcome(Outcome::Lose), 0);
        assert_eq!(super::score_from_outcome(Outcome::Draw), 3);
        assert_eq!(super::score_from_outcome(Outcome::Win), 6);
    }

    #[test]
    fn shape_scope() {
        assert_eq!(super::score_from_shape(Shape::Rock), 1);
        assert_eq!(super::score_from_shape(Shape::Paper), 2);
        assert_eq!(super::score_from_shape(Shape::Scissors), 3);
    }

    #[test]
    fn score() {
        assert_eq!(super::score_round(Shape::Rock, Shape::Rock), 3 + 1);
        assert_eq!(super::score_round(Shape::Rock, Shape::Paper), 0 + 1);
        assert_eq!(super::score_round(Shape::Rock, Shape::Scissors), 6 + 1);
        assert_eq!(super::score_round(Shape::Paper, Shape::Rock), 6 + 2);
        assert_eq!(super::score_round(Shape::Paper, Shape::Paper), 3 + 2);
        assert_eq!(super::score_round(Shape::Paper, Shape::Scissors), 0 + 2);
        assert_eq!(super::score_round(Shape::Scissors, Shape::Rock), 0 + 3);
        assert_eq!(super::score_round(Shape::Scissors, Shape::Paper), 6 + 3);
        assert_eq!(super::score_round(Shape::Scissors, Shape::Scissors), 3 + 3);
    }

    #[test]
    fn shapes_from_line() {
        assert_eq!(super::shapes_from_line("A Y".to_string()), Some(Round(Shape::Paper, Shape::Rock)));
        
        assert_eq!(super::shapes_from_line("".to_string()), None);
    }

    #[test]
    fn example_a() {
        let lines = [
            "A Y",
            "B X",
            "C Z"
        ].map(|line| line.to_string());

        let actual = super::calculate_total_score_from_shape(Box::new(lines.into_iter()));
        let expected = 6 + 2
                     + 1 + 0
                     + 3 + 3;

        assert_eq!(expected, 15);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_b() {
        let lines = [
            "A Y",
            "B X",
            "C Z"
        ].map(|line| line.to_string());

        let actual = super::calculate_total_score_from_outcome(Box::new(lines.into_iter()));
        let expected = 3 + 1
                     + 0 + 1
                     + 6 + 1;

        assert_eq!(expected, 12);
        assert_eq!(actual, expected);
    }

}