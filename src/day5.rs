use std::collections::VecDeque;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone)]
struct Container(char);

impl FromStr for Container {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if !trimmed.starts_with('[') && !trimmed.ends_with(']') {
            return Err(());
        }

        let c = s.chars().nth(1).unwrap();
        Ok(Self(c))
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    amount: i32,
    source: usize,
    dest: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // I should probably do better validation of the string...
        let parts: Vec<String> = s.split(' ').map(|p| p.to_string()).collect();
        let amount = parts.get(1).map(|i| i.parse::<i32>().unwrap()).unwrap();
        let source = parts.get(3).map(|s| s.parse::<usize>().unwrap()).unwrap();
        let dest = parts.get(5).map(|s| s.parse::<usize>().unwrap()).unwrap();

        let instruction = Self {
            amount,
            source,
            dest,
        };

        //dbg!(&instruction);

        Ok(instruction)
    }
}

fn build_container_map(s: &str) -> BTreeMap<usize, VecDeque<Container>> {
    let mut container_map: BTreeMap<usize, VecDeque<Container>> = BTreeMap::new();
    let lines: Vec<&str> = s.lines().collect();

    let container_identifiers: Vec<usize> = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    for c in container_identifiers {
        container_map.insert(c, VecDeque::new());
    }

    for line in &lines[..lines.len() - 1] {
        // substract 1 so we don't try to read the container identifier line
        let characters: Vec<char> = line.chars().collect();
        for (iteration, container_chars) in characters.chunks(4).enumerate() {
            let container_string: String = container_chars.iter().collect();

            // no container at this level, keep going
            if container_string.trim().is_empty() {
                continue;
            } else {
                let container = Container::from_str(&container_string).unwrap();
                container_map
                    .entry(iteration + 1)
                    .and_modify(|containers| containers.push_back(container));
            }
        }
    }

    container_map
}

fn build_instruction_list(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .collect()
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (BTreeMap<usize, VecDeque<Container>>, Vec<Instruction>) {
    let (container_diagram, raw_instructions) = input.split_once("\n\n").unwrap();

    let container_map = build_container_map(container_diagram);
    let instructions = build_instruction_list(raw_instructions);

    (container_map, instructions)
}

#[aoc(day5, part1)]
fn part1(input: &(BTreeMap<usize, VecDeque<Container>>, Vec<Instruction>)) -> String {
    let mut map_and_instructions = input.to_owned();
    for instruction in map_and_instructions.1 {
        for _ in 1..=instruction.amount {
            if let Some(source_container) = map_and_instructions
                .0
                .get_mut(&instruction.source)
                .unwrap()
                .pop_front()
            {
                // println!(
                //     "{}: moving {:?} from {:?} to {:?}",
                //     i, &source_container, &instruction.source, &instruction.dest
                // );
                map_and_instructions
                    .0
                    .get_mut(&instruction.dest)
                    .unwrap()
                    .push_front(source_container);
            }
        }
        // println!("-- AFTER INSTRUCTIONS --");
        // dbg!(&map_and_instructions.0);
        // println!("------------------------")
    }
    // dbg!(&map_and_instructions.0);

    let top_containers: String = map_and_instructions
        .0
        .iter()
        .filter_map(|(_, stack)| stack.front())
        .map(|c: &Container| c.0.to_string())
        .collect();

    top_containers
}

#[aoc(day5, part2)]
fn part2(input: &(BTreeMap<usize, VecDeque<Container>>, Vec<Instruction>)) -> String {
    let mut map_and_instructions = input.to_owned();
    for instruction in map_and_instructions.1 {
        let mut intermediary_stack: VecDeque<Container> = VecDeque::new();

        for _ in 1..=instruction.amount {
            if let Some(source_container) = map_and_instructions
                .0
                .get_mut(&instruction.source)
                .unwrap()
                .pop_front()
            {
                // println!(
                //     "{}: moving {:?} from {:?} to {:?}",
                //     i, &source_container, &instruction.source, &instruction.dest
                // );
                intermediary_stack.push_back(source_container);
            }
        }

        while let Some(container) = intermediary_stack.pop_back() {
            map_and_instructions
                .0
                .get_mut(&instruction.dest)
                .unwrap()
                .push_front(container);
        }

        // println!("-- AFTER INSTRUCTIONS --");
        // dbg!(&map_and_instructions.0);
        // println!("------------------------")
    }

    // dbg!(&map_and_instructions.0);

    let top_containers: String = map_and_instructions
        .0
        .iter()
        .filter_map(|(_, stack)| stack.front())
        .map(|c: &Container| c.0.to_string())
        .collect();

    top_containers
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!("CMZ", part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!("MCD", part2(&parsed_input))
    }
}
