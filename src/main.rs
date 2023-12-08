pub mod aoc2022;
pub mod aoc2023;
use std::fs;
use std::time::Instant;

macro_rules! add_day {
    ($day:ident, $year:ident ) => {
        println!("\n");
        println!(stringify!($day));
        println!("-------------------------------------------------------------");
        let file = fs::read_to_string(format!("src/{}/input/{}",
                                        stringify!($year),stringify!($day)))
                                        .expect("file not found");
        let now = Instant::now();
        println!("Part 1: {}", $year::$day::part_1(file.clone()));
        let elapsed = now.elapsed();
        println!("took: {:.2?}", elapsed);
        let now = Instant::now();
        println!("Part 2: {}", $year::$day::part_2(file.clone()));
        let elapsed = now.elapsed();
        println!("took: {:.2?}", elapsed);
        println!("-------------------------------------------------------------");
    };
}

fn main() {
    println!("Advent of Code 2022");
    add_day!(day1, aoc2022);
    add_day!(day2, aoc2022);
    add_day!(day3, aoc2022);
    add_day!(day4, aoc2022);
    add_day!(day5, aoc2022);
    add_day!(day6, aoc2022);
    add_day!(day7, aoc2022);
    add_day!(day8, aoc2022);
    add_day!(day9, aoc2022);
    add_day!(day10, aoc2022);

    println!("\nAdvent of Code 2023");
    add_day!(day1, aoc2023);
    add_day!(day2, aoc2023);
    add_day!(day3, aoc2023);
    add_day!(day4, aoc2023);
    // add_day!(day5, aoc2023);
    add_day!(day6, aoc2023);
    add_day!(day7, aoc2023);
    add_day!(day8, aoc2023);
}
