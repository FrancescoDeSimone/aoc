fn check_direction(line: &Vec<i32>) -> bool {
    let same_direction = line[0] < line[1];
    for index in 1..line.len() {
        if same_direction != (line[index - 1] < line[index]) {
            return false;
        }
    }
    true
}
fn check_abs(line: &Vec<i32>) -> bool {
    for index in 1..line.len() {
        let abs = line[index - 1].abs_diff(line[index]);
        if abs > 3 || abs < 1 {
            return false;
        }
    }
    true
}

pub fn part_1(input: String) -> usize {
    input
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(check_direction)
        .filter(check_abs)
        .count()
}

pub fn part_2(input: String) -> usize {
    input
        .lines()
        .map(|e| {
            e.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            if check_direction(line) && check_abs(line) {
                return true;
            }
            for i in 0..line.len() {
                let mut line_mut = (*line).clone();
                line_mut.remove(i);
                if check_direction(&line_mut) && check_abs(&line_mut) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[test]
fn check_part1() {
    let oracle = 2;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 4;
    let input = std::fs::read_to_string("src/aoc2024/tests/day2.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
