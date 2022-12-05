use std::{collections::HashSet, ops::Range, str::FromStr};

#[derive(Debug)]
struct Assignment {
    start: i32,
    end: i32,
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Assignment {
            start: start.parse::<i32>().unwrap(),
            end: end.parse::<i32>().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Pair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = s
            .split(',')
            .map(|a| Assignment::from_str(a.trim()).unwrap());

        Ok(Pair {
            first: pair.next().unwrap(),
            second: pair.next().unwrap(),
        })
    }
}

impl Pair {
    fn fully_contains_exists(&self) -> bool {
        if self.first.start >= self.second.start && self.first.end <= self.second.end {
            return true;
        }

        if self.second.start >= self.first.start && self.second.end <= self.first.end {
            return true;
        }

        false
    }

    fn has_overlap(&self) -> bool {
        if self.fully_contains_exists() {
            return true;
        }

        let first_range: HashSet<i32> = Range {
            start: self.first.start,
            end: self.first.end + 1,
        }
        .collect();

        let second_range: HashSet<i32> = Range {
            start: self.second.start,
            end: self.second.end + 1,
        }
        .collect();

        let intersection: HashSet<_> = first_range.intersection(&second_range).collect();

        !intersection.is_empty()
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Pair> {
    input.lines().map(|l| Pair::from_str(l).unwrap()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.fully_contains_exists()).count()
}

#[aoc(day4, part2)]
fn part2(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.has_overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(2, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(4, part2(&parsed_input))
    }
}
