pub fn part_1(input: String) -> usize {
    0
}

pub fn part_2(input: String) -> usize {
    0
}

#[test]
#[ignore]
fn check_part1() {
    let oracle = 136;
    let input = std::fs::read_to_string("src/aoc2023/tests/day15").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
#[ignore]
fn check_part2() {
    let oracle = 64;
    let input = std::fs::read_to_string("src/aoc2023/tests/day15").unwrap();
    assert_eq!(oracle, part_2(input));
}
