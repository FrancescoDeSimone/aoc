pub mod aoc2022;
use aoc2022::solutions;
use std::fs;

fn main() {
    println!("Day 1");
    let file = fs::read_to_string("src/aoc2022/day1").expect("day1 file not found");
    println!("Part 1: {}", solutions::day1_1(file.clone()));
    println!("Part 2: {}", solutions::day1_2(file.clone()));

    println!("Day 2");
    let file = fs::read_to_string("src/aoc2022/day2").expect("day2 file not found");
    println!("Part 1: {}", solutions::day2_1(file.clone()));
    println!("Part 2: {}", solutions::day2_2(file.clone()));

    println!("Day 3");
    let file = fs::read_to_string("src/aoc2022/day3").expect("day3 file not found");
    println!("Part 1: {}", solutions::day3_1(file.clone()));
    println!("Part 2: {}", solutions::day3_2(file.clone()));

    println!("Day 4");
    let file = fs::read_to_string("src/aoc2022/day4").expect("day4 file not found");
    println!("Part 1: {}", solutions::day4_1(file.clone()));
    println!("Part 2: {}", solutions::day4_2(file.clone()));
}
