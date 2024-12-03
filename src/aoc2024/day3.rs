use fancy_regex::{Captures, Regex};

pub fn part_1(input: String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum = 0;
    for captures in re.captures_iter(&input) {
        let captures = captures.unwrap();
        let left: i32 = captures[1].parse().expect("Failed to parse left number");
        let right: i32 = captures[2].parse().expect("Failed to parse right number");
        sum += left * right;
    }
    sum
}
pub fn part_2(input: String) -> i32 {
    let input = input.replace("\n", "");
    let re = Regex::new(r"(?m)don't\(\).*?do\(\)|don't.*?$").unwrap();
    let input = re.replace_all(&input, "").to_string();
    part_1(input)
}

#[test]
fn check_part1() {
    let oracle = 161;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 48;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
