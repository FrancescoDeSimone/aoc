use std::collections::HashSet;

pub fn part_1(input: String) -> usize {
    let groups = input
        .split('\n')
        .map(|e| {
            e.split(',')
                .filter(|e| !e.is_empty())
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
        })
        .filter(|e| !e.is_empty())
        .collect::<Vec<Vec<String>>>();

    let mut counter = 0;
    for pair in groups {
        let (s1i, s1f) = pair[0].split_at(pair[0].find('-').unwrap());
        let (s2i, s2f) = pair[1].split_at(pair[1].find('-').unwrap());
        let s1: HashSet<usize> =
            (s1i.trim().parse().unwrap()..=s1f.replace("-", " ").trim().parse().unwrap()).collect();
        let s2: HashSet<usize> =
            (s2i.trim().parse().unwrap()..=s2f.replace("-", " ").trim().parse().unwrap()).collect();
        let inter = s1.intersection(&s2).count();
        counter += if inter == s1.len() || inter == s2.len() {
            1
        } else {
            0
        };
    }
    counter
}

pub fn part_2(input: String) -> usize {
    let groups = input
        .split('\n')
        .map(|e| {
            e.split(',')
                .filter(|e| !e.is_empty())
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
        })
        .filter(|e| !e.is_empty())
        .collect::<Vec<Vec<String>>>();

    let mut counter = 0;
    for pair in groups {
        let (s1i, s1f) = pair[0].split_at(pair[0].find('-').unwrap());
        let (s2i, s2f) = pair[1].split_at(pair[1].find('-').unwrap());
        let s1: HashSet<usize> =
            (s1i.trim().parse().unwrap()..=s1f.replace("-", " ").trim().parse().unwrap()).collect();
        let s2: HashSet<usize> =
            (s2i.trim().parse().unwrap()..=s2f.replace("-", " ").trim().parse().unwrap()).collect();
        let inter = s1.intersection(&s2).count();
        counter += if inter > 0 { 1 } else { 0 };
    }
    counter
}
