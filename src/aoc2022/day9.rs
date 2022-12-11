use std::collections::HashMap;

pub fn part_1(input: String) -> usize {
    let mut tails: HashMap<(isize, isize), String> = HashMap::new();
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut prev_i = 0;
    let mut prev_j = 0;
    let mut tail_i = 0;
    let mut tail_j = 0;
    for token in input.split('\n').filter(|e| !e.is_empty()) {
        let (dir, count) = token.split_at(token.find(' ').unwrap());
        for _ in 0..count.trim().parse::<usize>().unwrap() {
            j = match dir {
                "U" => j - 1,
                "D" => j + 1,
                _ => j,
            };
            i = match dir {
                "L" => i - 1,
                "R" => i + 1,
                _ => i,
            };
            if (i - tail_i).abs() > 1 || (j - tail_j).abs() > 1 {
                tails.insert((prev_i, prev_j), "T".to_string());
                tail_i = prev_i;
                tail_j = prev_j;
            }
            prev_i = i;
            prev_j = j;
        }
    }
    tails.into_iter().count()
}

pub fn part_2(input: String) -> usize {
    let mut tails: HashMap<(isize, isize), String> = HashMap::new();
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut prev_i = 0;
    let mut prev_j = 0;
    let mut tail_i = 0;
    let mut tail_j = 0;
    for token in input.split('\n').filter(|e| !e.is_empty()) {
        let (dir, count) = token.split_at(token.find(' ').unwrap());
        for _ in 0..count.trim().parse::<usize>().unwrap() {
            j = match dir {
                "U" => j - 1,
                "D" => j + 1,
                _ => j,
            };
            i = match dir {
                "L" => i - 1,
                "R" => i + 1,
                _ => i,
            };
            if (i - tail_i).abs() > 1 || (j - tail_j).abs() > 1 {
                tails.insert((prev_i, prev_j), "T".to_string());
                tail_i = prev_i;
                tail_j = prev_j;
            }
            prev_i = i;
            prev_j = j;
        }
    }
    tails.into_iter().count()
}
