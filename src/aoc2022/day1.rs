use std::collections::BinaryHeap;

pub fn part_1(input: String) -> usize {
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

pub fn part_2(input: String) -> usize {
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
