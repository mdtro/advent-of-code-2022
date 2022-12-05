use std::str::FromStr;

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unrecognized hand shape"),
        };

        Ok(shape)
    }
}

impl From<&Shape> for i32 {
    fn from(shape: &Shape) -> i32 {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    opponent: Shape,
    mine: Shape,
}

impl Round {
    fn score(&self) -> i32 {
        match (&self.opponent, &self.mine) {
            (Shape::Rock, Shape::Rock) => 1 + 3,
            (Shape::Rock, Shape::Paper) => 2 + 6,
            (Shape::Rock, Shape::Scissors) => 3,
            (Shape::Paper, Shape::Rock) => 1,
            (Shape::Paper, Shape::Paper) => 2 + 3,
            (Shape::Paper, Shape::Scissors) => 3 + 6,
            (Shape::Scissors, Shape::Rock) => 1 + 6,
            (Shape::Scissors, Shape::Paper) => 2,
            (Shape::Scissors, Shape::Scissors) => 3 + 3,
        }
    }
}

// Part 2
#[derive(Debug, Clone)]
enum TargetResult {
    Lose,
    Win,
    Draw,
}

// This is hacky, but I don't want to re-write part 1's code lol.
impl From<&Shape> for TargetResult {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Rock => TargetResult::Lose,
            Shape::Paper => TargetResult::Draw,
            Shape::Scissors => TargetResult::Win,
        }
    }
}

impl From<&TargetResult> for i32 {
    fn from(target_result: &TargetResult) -> i32 {
        match target_result {
            TargetResult::Lose => 0,
            TargetResult::Win => 6,
            TargetResult::Draw => 3,
        }
    }
}

impl TargetResult {
    fn determine_play(&self, opponent_shape: &Shape) -> Shape {
        match (self, opponent_shape) {
            (TargetResult::Lose, Shape::Rock) => Shape::Scissors,
            (TargetResult::Lose, Shape::Paper) => Shape::Rock,
            (TargetResult::Lose, Shape::Scissors) => Shape::Paper,
            (TargetResult::Win, Shape::Rock) => Shape::Paper,
            (TargetResult::Win, Shape::Paper) => Shape::Scissors,
            (TargetResult::Win, Shape::Scissors) => Shape::Rock,
            (TargetResult::Draw, Shape::Rock) => Shape::Rock,
            (TargetResult::Draw, Shape::Paper) => Shape::Paper,
            (TargetResult::Draw, Shape::Scissors) => Shape::Scissors,
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|r| {
            let shapes = r.split_once(' ').unwrap();
            Round {
                opponent: Shape::from_str(shapes.0).unwrap(),
                mine: Shape::from_str(shapes.1).unwrap(),
            }
        })
        .collect()
}

// First Column
// Rock - A
// Paper - B
// Scissors - C

// Second Colum
// Rock - X
// Paper - Y
// Scissors - Z

// Round Score = shape selected + outcome
// Rock: 1, Paper: 2, Scissors: 3
// outcome -- lost: 0, draw: 3, won: 6

#[aoc(day2, part1)]
fn part1(input: &Vec<Round>) -> i32 {
    input.iter().map(|r| r.score()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Round>) -> i32 {
    input
        .iter()
        .map(|r| {
            let target_result = TargetResult::from(&r.mine);
            let my_shape = target_result.determine_play(&r.opponent);
            let score: i32 = i32::from(&my_shape) + i32::from(&target_result);
            score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(15, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(12, part2(&parsed_input))
    }
}
