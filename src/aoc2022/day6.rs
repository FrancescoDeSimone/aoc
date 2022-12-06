use std::collections::{HashSet, VecDeque};

pub fn part_1(input: String) -> usize {
    let mut sliding_window = VecDeque::new();
    let mut index = 0;
    for i in input.chars() {
        sliding_window.push_front(i);
        index += 1;
        if sliding_window.len() == 4 {
            let mut uniq = HashSet::new();
            if sliding_window
                .clone()
                .into_iter()
                .all(move |x| uniq.insert(x))
            {
                break;
            }
            sliding_window.pop_back();
        }
    }
    index
}

pub fn part_2(input: String) -> usize {
    let mut sliding_window = VecDeque::new();
    let mut index = 0;
    for i in input.chars() {
        sliding_window.push_front(i);
        index += 1;
        if sliding_window.len() == 14 {
            let mut uniq = HashSet::new();
            if sliding_window
                .clone()
                .into_iter()
                .all(move |x| uniq.insert(x))
            {
                break;
            }
            sliding_window.pop_back();
        }
    }
    index
}
