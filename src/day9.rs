use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(' ').unwrap();
        let direction = match parts.0 {
            "U" => Self::Up(parts.1.parse::<i32>().unwrap()),
            "D" => Self::Down(parts.1.parse::<i32>().unwrap()),
            "R" => Self::Right(parts.1.parse::<i32>().unwrap()),
            "L" => Self::Left(parts.1.parse::<i32>().unwrap()),
            _ => panic!("unknown instruction: {}", s),
        };
        Ok(direction)
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: Coord,
    tail: Coord,
}

impl Rope {
    fn process_instruction(&mut self, instruction: &Instruction) {
        // move head
        while let Some(instruct) = self.head.process_instruction(instruction.clone()) {
            // calculate distance
            let distance = self.tail.distance_from(&self.head);

            // move tail, if needed
            if distance >= 2.0_f32 {
                println!(
                    "tail needs move {:?}; distance {:?}; rope {:?}",
                    &instruction, distance, &self
                );
                self.move_tail();
                println!("rope after tail move: {:?}", &self);
            } else {
                println!(
                    "tail does not need move {:?}; distance {:?}; rope {:?}",
                    &instruction, distance, &self
                );
            }
        }
    }

    fn move_tail(&mut self) {
        let midpoint_x = (self.tail.x + self.head.x) / 2;
        let midpoint_y = (self.tail.y + self.head.y) / 2;

        self.tail.x = midpoint_x;
        self.tail.y = midpoint_y;
    }
}

#[derive(Debug, Default)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance_from(&self, other: &Coord) -> f32 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt()
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Option<Instruction> {
        match instruction {
            Instruction::Up(y) => {
                if y > 0 {
                    self.y += 1;
                    return Some(Instruction::Up(y - 1));
                } else {
                    return None;
                }
            }
            Instruction::Down(y) => {
                if y > 0 {
                    self.y -= 1;
                    return Some(Instruction::Down(y - 1));
                } else {
                    return None;
                }
            }
            Instruction::Left(x) => {
                if x > 0 {
                    self.x -= 1;
                    return Some(Instruction::Left(x - 1));
                } else {
                    return None;
                }
            }
            Instruction::Right(x) => {
                if x > 0 {
                    self.x += 1;
                    return Some(Instruction::Right(x - 1));
                } else {
                    return None;
                }
            }
        };
        unreachable!()
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let mut rope = Rope::default();
    for instruction in instructions {
        rope.process_instruction(instruction);
    }
    todo!()
}

#[aoc(day9, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(13, part1(&parsed_input))
    }

    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(999999, part2(&parsed_input))
    }
}
