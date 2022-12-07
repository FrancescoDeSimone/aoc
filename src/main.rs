pub mod aoc2022;
use std::fs;

fn main() {
    println!("part1");
    let file = fs::read_to_string("src/aoc2022/input/day1").expect("file not found");
    println!("Part 1: {}", aoc2022::day1::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day1::part_2(file.clone()));

    println!("part2");
    let file = fs::read_to_string("src/aoc2022/input/day2").expect("file not found");
    println!("Part 1: {}", aoc2022::day2::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day2::part_2(file.clone()));

    println!("part3");
    let file = fs::read_to_string("src/aoc2022/input/day3").expect("file not found");
    println!("Part 1: {}", aoc2022::day3::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day3::part_2(file.clone()));

    println!("part4");
    let file = fs::read_to_string("src/aoc2022/input/day4").expect("file not found");
    println!("Part 1: {}", aoc2022::day4::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day4::part_2(file.clone()));

    println!("part5");
    let file = fs::read_to_string("src/aoc2022/input/day5").expect("file not found");
    println!("Part 1: {}", aoc2022::day5::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day5::part_2(file.clone()));

    println!("part6");
    let file = fs::read_to_string("src/aoc2022/input/day6").expect("file not found");
    println!("Part 1: {}", aoc2022::day6::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day6::part_2(file.clone()));

    println!("part7");
    let file = fs::read_to_string("src/aoc2022/input/day7").expect("file not found");
    println!("Part 1: {}", aoc2022::day7::part_1(file.clone()));
    println!("Part 2: {}", aoc2022::day7::part_2(file.clone()));
}
