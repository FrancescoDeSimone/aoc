use rayon::prelude::*;
fn check_valid(sequence: &str, groups: &Vec<usize>) -> u64 {
    let consecutive_damage = sequence.split(".")
                                     .filter(|x| !x.is_empty())
                                     .map(|x| x.chars().count())
                                     .collect::<Vec<usize>>();
    (consecutive_damage == *groups) as u64
}

pub fn part_1(input: String) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        let (sequence, groups) = line.split_once(" ").unwrap();
        let groups = groups.split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let question_marks:i32 = sequence.chars().filter(|e| *e == '?').count() as i32;
        res += (0..(usize::pow(2, question_marks as u32))).into_par_iter().fold(|| 0_u64, |res, permutation| {
            let mut seq_perm: Vec<char> = sequence.chars().collect();
            let mut current_perm = permutation;
            let mut remaining_question_marks = question_marks;
            for c in seq_perm.iter_mut() {
                if remaining_question_marks == 0 {
                    break;
                }
                if *c == '?' {
                    let bit = current_perm & 1;
                    current_perm >>= 1;
                    *c = if bit == 0 { '.' } else { '#' };
                    remaining_question_marks -= 1;
                }
            }
            let seq_str: String = seq_perm.iter().collect();
            res + check_valid(&seq_str, &groups)
        }).sum::<u64>();
    }
    res
}


pub fn part_2(input: String) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        let (sequence, groups) = line.split_once(" ").unwrap();
        let sequence = sequence.to_string();
        let groups = groups.split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut sequence = (sequence+"?").repeat(5);
        sequence.pop();
        let groups = groups.repeat(5);
        let question_marks:i32 = sequence.chars().filter(|e| *e == '?').count() as i32;
        res += (0..(usize::pow(2, question_marks as u32))).into_par_iter().fold(|| 0_u64, |res, permutation| {
            let mut seq_perm = sequence.to_string();
            for n in 0..question_marks {
                seq_perm = if permutation >> n & 1 == 0 {
                    seq_perm.replacen('?', ".",1)
                } else {
                        seq_perm.replacen('?', "#",1)
                    };
            }
            res + check_valid(&seq_perm, &groups)
        }).sum::<u64>();
    }
    res
}

#[test]
fn check_part1(){
    let oracle = 21;
    let input = std::fs::read_to_string("src/aoc2023/tests/day12").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 525152;
    let input = std::fs::read_to_string("src/aoc2023/tests/day12").unwrap();
    assert_eq!(oracle, part_2(input));
}
