use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::usize;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn parse(input: &str) -> (Vec<Vec<&str>>, Vec<(&str, &str)>) {
    let (ordering_rules, pages_to_produce) = input.split_once("\n\n").unwrap();
    let pages_to_produce = pages_to_produce
        .lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rules: Vec<(_, _)> = ordering_rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();
    (pages_to_produce, rules)
}

fn check(page: &str, pages_to_check: &Vec<&str>, rules: &Vec<(&str, &str)>) -> bool {
    let rule_filtered: Vec<_> = rules.par_iter().filter(|r| r.1 == page).collect();
    for check in pages_to_check {
        if rule_filtered.par_iter().filter(|r| r.0 == *check).count() > 0 {
            return false;
        }
    }
    true
}

fn topological_sort<'a>(
    pages: &'a Vec<&'a str>,
    rules: &'a Vec<(&'a str, &'a str)>,
) -> Vec<&'a str> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, i32> = HashMap::new();

    for (x, y) in rules {
        graph.entry(x).or_insert(Vec::new()).push(y);
        *in_degree.entry(y).or_insert(0) += 1;
        in_degree.entry(x).or_insert(0);
    }

    for &page in pages {
        in_degree.entry(page).or_insert(0);
    }

    let mut sorted_pages = Vec::new();
    let mut queue: VecDeque<&str> = pages
        .iter()
        .filter(|&page| *in_degree.get(*page).unwrap_or(&0) == 0)
        .cloned()
        .collect();
    println!("Initial queue: {:?}", queue);

    while let Some(page) = queue.pop_front() {
        sorted_pages.push(page);
        if let Some(neighbors) = graph.get(page) {
            for &neighbor in neighbors {
                let count = in_degree.entry(neighbor).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted_pages
}

pub fn part_1(input: String) -> usize {
    let (pages_to_produce, rules) = parse(&input);
    pages_to_produce
        .into_iter()
        .filter(|pages| {
            pages
                .windows(2)
                .all(|window| check(window[0], &window[1..].to_vec(), &rules))
        })
        .map(|pages| pages[pages.len() / 2].parse::<usize>().unwrap())
        .sum()
}

pub fn part_2(input: String) -> usize {
    let (pages_to_produce, rules) = parse(&input);
    pages_to_produce
        .into_iter()
        .filter(|pages| {
            pages
                .windows(2)
                .any(|window| !check(window[0], &window[1..].to_vec(), &rules))
        })
        .map(|pages| {
            let sorted_pages = pages; //topological_sort(&pages, &rules);
            sorted_pages[sorted_pages.len() / 2]
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

#[test]
fn check_part1() {
    let oracle = 143;
    let input = std::fs::read_to_string("src/aoc2024/tests/day5.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 123;
    let input = std::fs::read_to_string("src/aoc2024/tests/day5.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
