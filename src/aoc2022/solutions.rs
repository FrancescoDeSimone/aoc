use std::collections::{BinaryHeap, HashMap};
use regex::Regex;

pub fn day1_1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|e| e.trim().parse::<usize>().unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .max()
        .unwrap()
}

pub fn day1_2(input: String) -> usize {
    //    let mut max_arr: [usize; 3] = [0; 3];
    //    let mut collection: usize = 0;
    //    for number in input.split('\n') {
    //        if number == "" {
    //            max_arr[0] = std::cmp::max(max_arr[0], collection);
    //            max_arr.sort();
    //            collection = 0;
    //        } else {
    //            let n: usize = number.trim().parse().unwrap();
    //            collection += n;
    //        }
    //    }
    //    max_arr.into_iter().reduce(|a, b| a + b).unwrap()
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|e| e.trim().parse::<usize>().unwrap())
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .collect::<BinaryHeap<usize>>()
        .into_vec()
        .into_iter()
        .take(3)
    .sum::<usize>()
}

pub fn day2_1(input: String) -> usize {
    let points_table = HashMap::from([
        ("A", HashMap::from([("Y", 8), ("X", 4), ("Z", 3)])),
        ("B", HashMap::from([("Y", 5), ("X", 1), ("Z", 9)])),
        ("C", HashMap::from([("Y", 2), ("X", 7), ("Z", 6)])),
        ]);
    let mut score = 0;
    for game in input.split('\n').filter(|x| !x.is_empty()) {
        let (enemy, me) = game.split_at(game.find(' ').unwrap());
        score += points_table[enemy.trim()][me.trim()];
    }
    score
}

pub fn day2_2(input: String) -> usize {
    let points_table = HashMap::from([
        ("A", HashMap::from([("Y", 4), ("X", 3), ("Z", 8)])),
        ("B", HashMap::from([("Y", 5), ("X", 1), ("Z", 9)])),
        ("C", HashMap::from([("Y", 6), ("X", 2), ("Z", 7)])),
        ]);
    let mut score = 0;
    for game in input.split('\n').filter(|x| !x.is_empty()) {
        let (enemy, me) = game.split_at(game.find(' ').unwrap());
        score += points_table[enemy.trim()][me.trim()];
    }
    score
}

pub fn day3_1(input: String) -> usize {
    let mut priority: usize = 0;
    for rucksacks in input.split('\n').filter(|x| !x.is_empty()) {
        let (sack_l, sack_r) = rucksacks.split_at(rucksacks.len() / 2);
        for e in sack_l.chars() {
            if sack_r.contains(e) {
                if e.is_lowercase() {
                    priority += 1 + (e as usize - 'a' as usize);
                } else {
                    priority += 27 + (e as usize - 'A' as usize);
                }
                break;
            }
        }
    }
    priority
}

pub fn day3_2(input: String) -> usize {
    let mut priority: usize = 0;
    let group_regex = Regex::new(r"(?m).*?\n.*?\n.*?\n").unwrap();
    for group in group_regex.captures_iter(&input) {
        let rucksacks = group.get(0).unwrap().as_str().split('\n').filter(|x| !x.is_empty()).collect::<Vec<_>>();
        for c in rucksacks[0].chars() {
            if rucksacks[1].contains(c) && rucksacks[2].contains(c) {
                if c.is_lowercase() {
                    priority += 1 + (c as usize - 'a' as usize);
                } else {
                    priority += 27 + (c as usize - 'A' as usize);
                }
                break;
            }
        }
    }
    priority
}
