use std::collections::HashMap;

pub fn part_1(input: String) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        input.lines().map(|l| l.split_once("   ").unwrap()).unzip();
    left.sort();
    let left = left.into_iter().map(|e| e.parse::<usize>().unwrap());
    right.sort();
    let right = right.into_iter().map(|e| e.parse::<usize>().unwrap());
    let diffs = left.into_iter().zip(right).map(|e| e.0.abs_diff(e.1));
    diffs.sum()
}

pub fn part_2(input: String) -> usize {
    let (left, right): (Vec<_>, Vec<_>) =
        input.lines().map(|l| l.split_once("   ").unwrap()).unzip();
    let left = left.into_iter().map(|e| e.parse::<usize>().unwrap());

    let mut count: HashMap<usize, usize> = HashMap::new();
    for e in right.into_iter().map(|e| e.parse::<usize>().unwrap()) {
        *count.entry(e).or_insert(0) += 1;
    }

    let scores = left.map(|e| e * count.get(&e).unwrap_or(&0));
    scores.sum()
}

#[test]
fn check_part1() {
    let oracle = 142;
    let input = std::fs::read_to_string("src/aoc2024/tests/day1.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 281;
    let input = std::fs::read_to_string("src/aoc2024/tests/day1.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
