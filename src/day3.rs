use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn priority_map() -> HashMap<char, i32> {
    let mut priorities = HashMap::new();

    let mut priority = 1;

    for l in 'a'..='z' {
        priorities.insert(l, priority);
        priority += 1;
    }

    for l in 'A'..='Z' {
        priorities.insert(l, priority);
        priority += 1;
    }

    priorities
}

#[derive(Debug, Clone)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
    full: HashSet<char>,
}

impl Rucksack {
    fn find_duplicate_item(&self) -> char {
        *self.first
            .intersection(&self.second)
            .next()
            .unwrap()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_index: usize = s.len() / 2;
        let first: HashSet<char> = s[..split_index].chars().collect();
        let second: HashSet<char> = s[split_index..].chars().collect();
        let full: HashSet<char> = s.chars().collect();
        Ok(Rucksack {
            first,
            second,
            full,
        })
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|ruck| Rucksack::from_str(ruck).unwrap())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Rucksack>) -> i32 {
    let priorities = priority_map();
    input
        .iter()
        .map(|r| {
            let dupe = r.find_duplicate_item();
            priorities.get(&dupe).unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Rucksack>) -> i32 {
    let priorities = priority_map();

    let mut sum = 0;

    for group in input.chunks(3) {
        let r1 = group[0].full.clone();
        let r2 = group[1].full.clone();
        let r3 = group[2].full.clone();

        let first_intersection: HashSet<_> = r1.intersection(&r2).collect();

        for c in first_intersection {
            if r3.contains(c) {
                sum += priorities.get(c).unwrap();
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(157, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(70, part2(&parsed_input))
    }
}
