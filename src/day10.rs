use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("addx") {
            let (_, amount) = s.split_once(' ').unwrap();
            Ok(Self::Addx(amount.parse::<i32>().unwrap()))
        } else {
            Ok(Self::Noop)
        }
    }
}

impl Instruction {
    fn get_num_of_cycles(&self) -> i32 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

#[derive(Debug)]
struct Cpu {
    register: i32,
    cycle: i32,
    signal_strengths: Vec<i32>,
    crt: String,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            register: 1,
            cycle: 0,
            signal_strengths: vec![],
            crt: "".to_string(),
        }
    }
}

impl Cpu {
    fn signal_strength(&self) -> i32 {
        self.cycle * self.register
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Addx(value) => {
                for c in 0..2 {
                    self.cycle += 1;
                    self.update_signal_strength();

                    if c == 1 {
                        // off by one... cycle 1: 0, cycle 2: 1
                        self.register += value;
                    }
                }
            }
            Instruction::Noop => {
                self.cycle += 1;
                self.update_signal_strength();
            }
        }
    }

    fn update_signal_strength(&mut self) {
        let cycles = [20, 60, 100, 140, 180, 220];
        if cycles.contains(&self.cycle) {
            let current_signal_strength = self.signal_strength();
            self.signal_strengths.push(current_signal_strength)
        }
    }

    fn build_screen(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            for c in 0..instruction.get_num_of_cycles() {
                let relative_pixel = (self.cycle + c) % 40;
                if ((self.register - 1)..=(self.register + 1)).contains(&relative_pixel) {
                    self.crt.push('#');
                } else {
                    self.crt.push(' '); //using a space here since I can't read with the dots
                }
            }

            self.cycle += instruction.get_num_of_cycles();
            match instruction {
                Instruction::Addx(value) => self.register += value,
                Instruction::Noop => {}
            }
        }
    }

    fn print_screen(&self) -> String {
        // TODO: refactor the below... there has to be a simpler way...
        self.crt
            .chars()
            .collect::<Vec<char>>()
            .chunks(40)
            .into_iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line.trim()).unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu::default();

    for i in instructions {
        cpu.process_instruction(i);
    }

    cpu.signal_strengths.iter().sum()
}

#[aoc(day10, part2)]
fn part2(instructions: &[Instruction]) -> String {
    let mut cpu = Cpu::default();
    cpu.build_screen(instructions);

    let screen = cpu.print_screen();
    format!("\n{}", screen)
}

mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(13140, part1(&parsed_input))
    }

    const PART2_OUTPUT: &str = "
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ";
    #[test]
    fn test_part2() {
        let parsed_input = input_generator(INPUT);
        assert_eq!(PART2_OUTPUT, part2(&parsed_input))
    }
}
