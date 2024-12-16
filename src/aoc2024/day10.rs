use std::collections::HashSet;

pub fn part_1(input: String) -> usize {
    let hmap: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let rows = hmap.len();
    let cols = hmap[0].len();

    let mut sum = 0;
    for r in 0..rows {
        for c in 0..cols {
            if hmap[r][c] == 0 {
                sum += count_reachable_nines(&hmap, r, c);
            }
        }
    }
    sum
}

fn count_reachable_nines(hmap: &[Vec<u8>], start_row: usize, start_col: usize) -> usize {
    let rows = hmap.len();
    let cols = hmap[0].len();
    let mut stack = vec![(start_row, start_col)];
    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        if hmap[x][y] == 9 {
            count += 1;
            continue;
        }

        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if hmap[nx][ny] == hmap[x][y] + 1 {
                    stack.push((nx, ny));
                }
            }
        }
    }
    count
}

pub fn part_2(input: String) -> usize {
    let hmap: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let rows = hmap.len();
    let cols = hmap[0].len();
    let mut zeros = input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut zeros, (row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, ch)| {
                if ch == '0' {
                    zeros.push((row_idx, col_idx));
                }
            });
            zeros
        });

    let mut sum = 0;
    while let Some((x, y)) = zeros.pop() {
        let mut stack = Vec::new();
        stack.push((x, y));
        let mut count = 0;
        while let Some((x, y)) = stack.pop() {
            let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dy) in directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if hmap[nx][ny] == hmap[x][y] + 1 {
                        if hmap[nx][ny] == 9 {
                            count += 1;
                        }
                        stack.push((nx, ny));
                    }
                }
            }
        }
        sum += count;
    }
    sum
}

#[test]
fn check_part1() {
    let oracle = 36;
    let input = std::fs::read_to_string("src/aoc2024/tests/day10.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 2858;
    let input = std::fs::read_to_string("src/aoc2024/tests/day10.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
