use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn split_number(num: u64) -> (u64, u64) {
    let size: u32 = (num as f64).log10().floor() as u32 + 1;
    let first = num / 10_u64.pow(size / 2);
    let second = num % 10_u64.pow(size / 2);
    (first, second)
}

fn process_stones(input: String, iterations: usize) -> usize {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();

    for _ in 0..iterations {
        stones = stones
            .par_iter()
            .fold(Vec::new, |mut acc, &stone| {
                match stone {
                    0 => acc.push(1),
                    s if ((s as f64).log10().floor() as u32 + 1) % 2 == 0 => {
                        let (stone_1, stone_2) = split_number(s);
                        acc.push(stone_1);
                        acc.push(stone_2);
                    }
                    _ => acc.push(stone * 2024),
                };
                acc
            })
            .flatten()
            .collect();
    }
    stones.len()
}

fn process_stones_dp(input: String, iterations: usize) -> usize {
    let mut stones: HashMap<u64, usize> = HashMap::new();
    for s in input.split_whitespace() {
        *stones.entry(s.parse().unwrap()).or_insert(0) += 1;
    }
    for _ in 0..iterations {
        let mut next_stones: HashMap<u64, usize> = HashMap::new();
        for (stone, count) in stones {
            match stone {
                0 => *next_stones.entry(1).or_insert(0) += count,
                s if ((s as f64).log10().floor() as u32 + 1) % 2 == 0 => {
                    let (stone_1, stone_2) = split_number(s);
                    *next_stones.entry(stone_1).or_insert(0) += count;
                    *next_stones.entry(stone_2).or_insert(0) += count;
                }
                _ => *next_stones.entry(stone * 2024).or_insert(0) += count,
            }
        }
        stones = next_stones;
    }

    stones.values().sum()
}

pub fn part_1(input: String) -> usize {
    process_stones_dp(input, 25)
}

pub fn part_2(input: String) -> usize {
    process_stones_dp(input, 75)
}
#[test]
fn check_part1() {
    let oracle = 36;
    let input = std::fs::read_to_string("src/aoc2024/tests/day11.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 2858;
    let input = std::fs::read_to_string("src/aoc2024/tests/day11.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
