pub fn part_1(input: String) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for j in 0..map[0].len() {
        for i in 1..map.len() {
            if map[i][j] == 'O' {
                let mut k = i;
                while k > 0 && map[k - 1][j] == '.' {
                    map[k - 1][j] = 'O';
                    map[k][j] = '.';
                    k -= 1;
                }
            }
        }
    }

    map.iter()
        .rev()
        .enumerate()
        .map(|(index, line)| (index + 1) * line.iter().filter(|c| **c == 'O').count())
        .sum()
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn tilts(grid: &mut Vec<Vec<char>>, direction: &Direction) {
    match direction {
        Direction::Up => {
            for j in 0..grid[0].len() {
                for i in 1..grid.len() {
                    if grid[i][j] == 'O' {
                        let mut row = i;
                        while row > 0 && grid[row - 1][j] == '.' {
                            grid[row - 1][j] = 'O';
                            grid[row][j] = '.';
                            row -= 1;
                        }
                    }
                }
            }
        }
        Direction::Down => {
            for j in 0..grid[0].len() {
                for i in (0..grid.len()).rev() {
                    if grid[i][j] == 'O' {
                        let mut row = i;
                        while row < grid.len() - 1 && grid[row + 1][j] == '.' {
                            grid[row + 1][j] = 'O';
                            grid[row][j] = '.';
                            row += 1;
                        }
                    }
                }
            }
        }
        Direction::Left => {
            for i in 0..grid.len() {
                for j in 1..grid[i].len() {
                    if grid[i][j] == 'O' {
                        let mut col = j;
                        while col > 0 && grid[i][col - 1] == '.' {
                            grid[i][col - 1] = 'O';
                            grid[i][col] = '.';
                            col -= 1;
                        }
                    }
                }
            }
        }
        Direction::Right => {
            for i in 0..grid.len() {
                for j in (0..grid[i].len()).rev() {
                    if grid[i][j] == 'O' {
                        let mut col = j;
                        while col < grid[0].len() - 1 && grid[i][col + 1] == '.' {
                            grid[i][col + 1] = 'O';
                            grid[i][col] = '.';
                            col += 1;
                        }
                    }
                }
            }
        }
    }
}

pub fn print_debug(map: &Vec<Vec<char>>) {
    println!("------------");
    for (index, row) in map.iter().enumerate() {
        print!("{:2} ", index + 1);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("------------");
}

pub fn part_2(input: String) -> usize {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen = vec![];
    let mut steps = 0;
    const ITERATIONS: usize = 1000000000;
    for _ in 0..ITERATIONS {
        steps += 1;
        seen.push(map.iter().fold(String::new(), |acc, line| {
            acc.to_string() + &line.iter().collect::<String>()
        }));
        tilts(&mut map, &Direction::Up);
        tilts(&mut map, &Direction::Left);
        tilts(&mut map, &Direction::Down);
        tilts(&mut map, &Direction::Right);
        let stop = map.iter().fold(String::new(), |acc, line| {
            acc.to_string() + &line.iter().collect::<String>()
        });
        if seen.contains(&stop) {
            break;
        }
    }
    let stop = map.iter().fold(String::new(), |acc, line| {
        acc.to_string() + &line.iter().collect::<String>()
    });
    let first = seen.iter().position(|s| s == &stop).unwrap();
    let map: Vec<Vec<char>> = seen[(ITERATIONS - first).rem_euclid(steps - first) + first]
        .chars()
        .collect::<Vec<char>>()
        .chunks(map[0].len())
        .map(|c| c.to_vec())
        .collect();

    map.iter()
        .rev()
        .enumerate()
        .map(|(index, line)| (index + 1) * line.iter().filter(|c| **c == 'O').count())
        .sum()
}

#[test]
fn check_part1() {
    let oracle = 136;
    let input = std::fs::read_to_string("src/aoc2023/tests/day14").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 64;
    let input = std::fs::read_to_string("src/aoc2023/tests/day14").unwrap();
    assert_eq!(oracle, part_2(input));
}
