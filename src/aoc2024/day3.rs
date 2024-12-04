use fancy_regex::Regex;

pub fn part_1(input: String) -> i32 {
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(&input)
        .filter_map(|captures| {
            let captures = captures.ok()?;
            let left = captures[1].parse::<i32>().ok()?;
            let right = captures[2].parse::<i32>().ok()?;
            Some(left * right)
        })
        .sum()
}
pub fn part_2(input: String) -> i32 {
    let input = input.replace("\n", "");
    let re = Regex::new(r"(?m)don't\(\).*?do\(\)|don't\(\).*?$").unwrap();
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
