use std::str::FromStr;

#[derive(Debug)]
struct Elf(Vec<i32>);

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: Vec<i32> = s
            .lines()
            .map(|i| i.trim().parse::<i32>().unwrap())
            .collect();

        Ok(Elf(calories))
    }
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Elf> {
    let elves: Vec<&str> = input.split("\n\n").collect();

    let elves_and_calories = elves
        .iter()
        .map(|elf| Elf::from_str(elf).unwrap())
        .collect();

    elves_and_calories
}

#[aoc(day1, part1)]
fn part1(input: &Vec<Elf>) -> i32 {
    input.iter().map(|e| e.0.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<Elf>) -> i32 {
    let mut calories: Vec<i32> = input.iter().map(|e| e.0.iter().sum()).collect();
    calories.sort();
    calories.reverse();

    calories[..=2].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(24000, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(45000, part2(&parsed_input))
    }
}
