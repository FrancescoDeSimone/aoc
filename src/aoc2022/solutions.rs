use std::collections::{BinaryHeap, HashMap};

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
    let mut game_map = HashMap::new();
    game_map.insert("A", HashMap::from([("Y", 8), ("X", 4), ("Z", 3)]));
    game_map.insert("B", HashMap::from([("Y", 5), ("X", 1), ("Z", 9)]));
    game_map.insert("C", HashMap::from([("Y", 2), ("X", 7), ("Z", 6)]));
    let mut score = 0;
    for game in input.split('\n').filter(|x| !x.is_empty()) {
        let (enemy, me) = game.split_at(game.find(' ').unwrap());
        score += game_map[enemy.trim()][me.trim()];
    }
    score
}

pub fn day2_2(input: String) -> usize {
    let mut game_map = HashMap::new();
    game_map.insert("A", HashMap::from([("Y", 4), ("X", 3), ("Z", 8)]));
    game_map.insert("B", HashMap::from([("Y", 5), ("X", 1), ("Z", 9)]));
    game_map.insert("C", HashMap::from([("Y", 6), ("X", 2), ("Z", 7)]));
    let mut score = 0;
    for game in input.split('\n').filter(|x| !x.is_empty()) {
        let (enemy, me) = game.split_at(game.find(' ').unwrap());
        score += game_map[enemy.trim()][me.trim()];
    }
    score
}
