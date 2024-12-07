use std::fs;
use std::path::Path;
use std::time::Instant;

const AOC_DIR: &str = "src";
const INPUT_DIR: &str = "input";
const FILE_EXT: &str = ".txt";

pub mod aoc2022;
pub mod aoc2023;
pub mod aoc2024;

macro_rules! add_day {
    ($day:ident, $year:ident) => {{
        let file_path = Path::new(AOC_DIR)
            .join(stringify!($year))
            .join(INPUT_DIR)
            .join(format!("{}{}", stringify!($day), FILE_EXT));
        let file_contents = fs::read_to_string(file_path)?;
        println!("\n");
        println!("{}", stringify!($day));
        println!("-------------------------------------------------------------");
        let now = Instant::now();
        println!("Part 1: {}", $year::$day::part_1(file_contents.clone()));
        let elapsed = now.elapsed();
        println!("took: {:.2?}", elapsed);
        let now = Instant::now();
        println!("Part 2: {}", $year::$day::part_2(file_contents.clone()));
        let elapsed = now.elapsed();
        println!("took: {:.2?}", elapsed);
        println!("-------------------------------------------------------------");
    }};
}

macro_rules! add_days {
    ($year:ident, $($day:ident),*) => {{
        println!("\nAdvent of Code {}", stringify!($year));
        $(
            add_day!($day, $year);
        )*
    }};
}

fn main() -> Result<(), std::io::Error> {
    //add_days!(aoc2022, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10);
    //add_days!(aoc2023, day1, day2, day3, day4, day6, day7, day8, day9, day10, day11, day14, day15);
    add_days!(aoc2024, day1, day2, day3, day4, day5, day6, day7);
    Ok(())
}
