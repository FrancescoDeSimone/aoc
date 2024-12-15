pub fn part_1(input: String) -> usize {
    0
}

pub fn part_2(input: String) -> usize {
    0
}

#[test]
fn check_part1() {
    let oracle = 1928;
    let input = std::fs::read_to_string("src/aoc2024/tests/day10.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 2858;
    let input = std::fs::read_to_string("src/aoc2024/tests/day10.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
