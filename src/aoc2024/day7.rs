fn concatenate(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let digits_b = (b as f64).log(10_f64).floor() as u32 + 1;
    a * 10u64.pow(digits_b) + b
}

fn _calibration_result(
    value: u64,
    equation: &[u64],
    index: usize,
    current_sum: u64,
    conc: bool,
) -> u64 {
    if index == equation.len() {
        return current_sum;
    }
    let v = equation[index];
    let sum_result = _calibration_result(value, equation, index + 1, current_sum + v, conc);
    let mult_result = _calibration_result(value, equation, index + 1, current_sum * v, conc);
    let mut concat_result = 0;
    if conc {
        concat_result = _calibration_result(
            value,
            equation,
            index + 1,
            concatenate(current_sum, v),
            conc,
        );
    }
    if sum_result == value || mult_result == value || concat_result == value {
        return value;
    }
    0
}

fn calibration_result(value: u64, equation: &[u64], concatenate: bool) -> u64 {
    _calibration_result(value, equation, 0, 0, concatenate)
}

pub fn part_1(input: String) -> u64 {
    let mut sum = 0;
    for line in input.lines().map(|line| {
        line.split_once(':')
            .map(|e| (e.0, e.1.split_whitespace().collect::<Vec<_>>()))
    }) {
        let line = line.unwrap();
        let (value, equation) = &line;
        let value = value.parse::<u64>().unwrap();
        let equation: Vec<u64> = equation.iter().map(|e| e.parse().unwrap()).collect();
        if calibration_result(value, &equation, false) == value {
            sum += value;
        }
    }
    sum
}

pub fn part_2(input: String) -> u64 {
    let mut sum = 0;
    for line in input.lines().map(|line| {
        line.split_once(':')
            .map(|e| (e.0, e.1.split_whitespace().collect::<Vec<_>>()))
    }) {
        let line = line.unwrap();
        let (value, equation) = &line;
        let value = value.parse::<u64>().unwrap();
        let equation: Vec<u64> = equation.iter().map(|e| e.parse().unwrap()).collect();
        if calibration_result(value, &equation, true) == value {
            sum += value;
        }
    }
    sum
}

#[test]
fn check_part1() {
    let oracle = 3749;
    let input = std::fs::read_to_string("src/aoc2024/tests/day7.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 11387;
    let input = std::fs::read_to_string("src/aoc2024/tests/day7.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
