use std::collections::HashSet;

struct Packet(Vec<char>);

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Packet {
    let chars: Vec<char> = input.chars().collect();
    Packet(chars)
}

#[aoc(day6, part1)]
fn part1(input: &Packet) -> usize {
    for (index, slice) in input.0.windows(4).enumerate() {
        let count_of_unique_chars = slice.iter().collect::<HashSet<_>>().len();
        if count_of_unique_chars == 4 {
            return index + 4;
        }
    }
    panic!("did not find packet marker!")
}

#[aoc(day6, part2)]
fn part2(input: &Packet) -> usize {
    for (index, slice) in input.0.windows(14).enumerate() {
        let count_of_unique_chars = slice.iter().collect::<HashSet<_>>().len();
        if count_of_unique_chars == 14 {
            return index + 14;
        }
    }
    panic!("did not find packet marker!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let parsed_input = input_generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(7, part1(&parsed_input))
    }

    #[test]
    fn test_part1_b() {
        let parsed_input = input_generator("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(5, part1(&parsed_input))
    }

    #[test]
    fn test_part1_c() {
        let parsed_input = input_generator("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(6, part1(&parsed_input))
    }

    #[test]
    fn test_part1_d() {
        let parsed_input = input_generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(10, part1(&parsed_input))
    }

    #[test]
    fn test_part1_e() {
        let parsed_input = input_generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(11, part1(&parsed_input))
    }

    #[test]
    fn test_part2_a() {
        let parsed_input = input_generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(19, part2(&parsed_input))
    }

    #[test]
    fn test_part2_b() {
        let parsed_input = input_generator("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(23, part2(&parsed_input))
    }

    #[test]
    fn test_part2_c() {
        let parsed_input = input_generator("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(23, part2(&parsed_input))
    }

    #[test]
    fn test_part2_d() {
        let parsed_input = input_generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(29, part2(&parsed_input))
    }

    #[test]
    fn test_part2_e() {
        let parsed_input = input_generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(26, part2(&parsed_input))
    }
}
