use std::collections::HashMap;

pub fn part_1(input: String) -> usize {
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

pub fn part_2(input: String) -> usize {
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
