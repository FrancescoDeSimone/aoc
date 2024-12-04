fn check_direction(
    table_input: &[Vec<char>],
    i: usize,
    j: usize,
    direction: (isize, isize),
    target: &str,
) -> bool {
    let (di, dj) = direction;

    for (k, c) in target.chars().enumerate() {
        let new_row = (i as isize + di * (k as isize + 1)) as usize;
        let new_col = (j as isize + dj * (k as isize + 1)) as usize;

        if new_row >= table_input.len()
            || new_col >= table_input[0].len()
            || table_input[new_row][new_col] != c
        {
            return false;
        }
    }
    true
}

pub fn part_1(input: String) -> usize {
    let table_input: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let directions = [
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];
    for i in 0..table_input.len() {
        for j in 0..table_input[i].len() {
            if table_input[i][j] == 'X' {
                for &direction in &directions {
                    sum += check_direction(&table_input, i, j, direction, "MAS") as usize;
                }
            }
        }
    }
    sum
}

pub fn part_2(input: String) -> usize {
    let table_input: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let d = [(-1, 1), (-1, -1), (1, -1), (1, 1)];
    for i in 0..table_input.len() {
        for j in 0..table_input[i].len() {
            if table_input[i][j] == 'A'
                && ((check_direction(&table_input, i, j, d[0], "M")
                    && check_direction(&table_input, i, j, d[2], "S"))
                    || (check_direction(&table_input, i, j, d[0], "S")
                        && check_direction(&table_input, i, j, d[2], "M")))
                && ((check_direction(&table_input, i, j, d[1], "M")
                    && check_direction(&table_input, i, j, d[3], "S"))
                    || (check_direction(&table_input, i, j, d[1], "S")
                        && check_direction(&table_input, i, j, d[3], "M")))
            {
                sum += 1;
            }
        }
    }
    sum
}

#[test]
fn check_part1() {
    let oracle = 161;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 48;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
