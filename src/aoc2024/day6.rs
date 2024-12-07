use std::collections::HashSet;

fn find_start(input: &[Vec<char>]) -> Option<(usize, usize)> {
    input.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter().enumerate().find_map(|(col_idx, &ch)| {
            if ch == '^' {
                Some((row_idx, col_idx))
            } else {
                None
            }
        })
    })
}

fn create_path(input: &Vec<Vec<char>>, mark: char) -> Option<Vec<Vec<char>>> {
    let mut input = input.clone();
    let (mut x, mut y) = find_start(&input).unwrap();
    let mut direction = 0;
    let mut visited = HashSet::new();
    loop {
        input[x][y] = mark;
        if !visited.insert((x, y, direction)) {
            return None;
        }

        let xx = x as i32;
        let yy = y as i32;
        let (nx, ny) = match direction {
            0 => (xx - 1, yy),
            1 => (xx, yy + 1),
            2 => (xx + 1, yy),
            3 => (xx, yy - 1),
            _ => unreachable!(),
        };
        if nx < 0 || ny < 0 || (nx as usize >= input.len()) || (ny as usize >= input[0].len()) {
            break;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if input[nx][ny] == '#' {
            direction = (direction + 1) % 4;
        } else {
            x = nx;
            y = ny;
        }
    }
    Some(input)
}

pub fn part_1(input: String) -> usize {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let input = create_path(&input, 'X').unwrap();
    input
        .iter()
        .map(|line| line.iter().filter(|&e| *e == 'X').count())
        .sum()
}

pub fn part_2(input: String) -> usize {
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let input_1 = create_path(&input, 'X').unwrap();
    let mut positions = Vec::new();
    for i in 0..input_1.len() {
        for j in 0..input_1[0].len() {
            if input_1[i][j] == 'X' {
                positions.push((i, j));
            }
        }
    }
    let start = find_start(&input).unwrap();

    let mut count = 0;
    for (x, y) in positions {
        if x == start.0 && y == start.1 {
            continue;
        }
        input[x][y] = '#';
        count += create_path(&input, 'X').is_none() as usize;
        input[x][y] = 'X';
    }
    count
}

#[test]
fn check_part1() {
    let oracle = 41;
    let input = std::fs::read_to_string("src/aoc2024/tests/day6.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 6;
    let input = std::fs::read_to_string("src/aoc2024/tests/day6.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
