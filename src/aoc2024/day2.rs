fn check_direction(line: &[i32]) -> bool {
    let same_direction = line[0] < line[1];
    line.windows(2)
        .all(|window| same_direction == (window[0] < window[1]))
}

fn check_abs(line: &[i32]) -> bool {
    line.windows(2).all(|window| {
        let abs_diff = window[0].abs_diff(window[1]);
        (1..=3).contains(&abs_diff)
    })
}

pub fn part_1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| check_direction(line) && check_abs(line))
        .count()
}
pub fn part_2(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            if check_direction(line) && check_abs(line) {
                return true;
            }

            (0..line.len()).any(|i| {
                let mut line_mut = line.to_vec();
                line_mut.remove(i);
                check_direction(&line_mut) && check_abs(&line_mut)
            })
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
