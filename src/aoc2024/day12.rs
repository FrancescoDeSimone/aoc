use std::collections::{HashSet, VecDeque};

fn part_1_check(new_i: isize, new_j: isize, x: usize, y: usize, input: &[Vec<char>]) -> bool {
    new_i < 0
        || new_i >= input.len() as isize
        || new_j < 0
        || new_j >= input[x].len() as isize
        || input[x][y] != input[new_i as usize][new_j as usize]
}

fn count_perimeter(input: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut perimeter_count = 0;

    for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let new_i = (x as isize) + *dx;
        let new_j = (y as isize) + *dy;
        if part_1_check(new_i, new_j, x, y, input) {
            perimeter_count += 1;
        }
    }
    perimeter_count
}

fn calculate_region(input: &[Vec<char>], x: usize, y: usize) -> (HashSet<(usize, usize)>, usize) {
    let mut vis = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_front((x, y));
    let mut perimeter = 0;

    while let Some((x, y)) = stack.pop_front() {
        if vis.insert((x, y)) {
            perimeter += count_perimeter(input, x, y);
        }

        for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let new_i = (x as isize) + *dx;
            let new_j = (y as isize) + *dy;
            if new_i < 0
                || new_i >= input.len() as isize
                || new_j < 0
                || new_j >= input[x].len() as isize
                || input[x][y] != input[new_i as usize][new_j as usize]
            {
                continue;
            }
            if !vis.contains(&(new_i as usize, new_j as usize)) {
                stack.push_front((new_i as usize, new_j as usize));
            }
        }
    }
    (vis, perimeter)
}

fn calculate_region_2(
    input: &[Vec<char>],
    x: usize,
    y: usize,
) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let mut vis = HashSet::new();
    let mut edges = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_front((x, y));

    while let Some((x, y)) = stack.pop_front() {
        vis.insert((x, y));
        for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let new_i = (x as isize) + *dx;
            let new_j = (y as isize) + *dy;
            if new_i < 0
                || new_i >= input.len() as isize
                || new_j < 0
                || new_j >= input[x].len() as isize
                || input[x][y] != input[new_i as usize][new_j as usize]
            {
                edges.insert((x, y));
                continue;
            }
            if !vis.contains(&(new_i as usize, new_j as usize)) {
                stack.push_front((new_i as usize, new_j as usize));
            }
        }
    }
    let mut edges = edges.into_iter().collect::<Vec<(usize, usize)>>();
    edges.sort();
    (vis, edges)
}
pub fn part_1(input: String) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|e| e.chars().collect()).collect();
    let mut sum = 0;
    let mut visited = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if !visited.contains(&(i, j)) {
                let region = calculate_region(&input, i, j);
                let (count, perimeter) = (region.0, region.1);
                sum += count.len() * perimeter;
                for (x, y) in count {
                    visited.insert((x, y));
                }
            }
        }
    }

    sum
}

fn count_sides(input: &[Vec<char>], region: &HashSet<(usize, usize)>) -> usize {
    let mut sides = 0;
    let rows = input.len();
    let cols = input[0].len();

    for &(r, c) in region {
        if r == 0 || !region.contains(&(r - 1, c)) {
            sides += 1;
        }
        if r == rows - 1 || !region.contains(&(r + 1, c)) {
            sides += 1;
        }
        if c == 0 || !region.contains(&(r, c - 1)) {
            sides += 1;
        }
        if c == cols - 1 || !region.contains(&(r, c + 1)) {
            sides += 1;
        }
    }
    sides
}
pub fn part_2(input: String) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|e| e.chars().collect()).collect();
    let mut visited = HashSet::new();
    let mut sum = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if !visited.contains(&(i, j)) {
                let (region, _) = calculate_region_2(&input, i, j);
                let size = count_sides(&input, &region);
                eprint!("region.len()={:?} ", region.len());
                eprintln!("size={:?}", size);
                sum += region.len() * size;

                for (x, y) in region {
                    visited.insert((x, y));
                }
            }
        }
    }
    sum
}

#[test]
fn check_part1() {
    let oracle = 1930;
    let input = std::fs::read_to_string("src/aoc2024/tests/day12.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 1206;
    let input = std::fs::read_to_string("src/aoc2024/tests/day12.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
