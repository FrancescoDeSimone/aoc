pub fn part_1(input: String) -> usize {
    input.split(",").fold(0, |hash, seq| {
        hash + seq
            .trim()
            .chars()
            .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
    })
}

fn aoc_hash(seq: &str) -> usize {
    seq.trim()
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

pub fn part_2(input: String) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for seq in input.split(",").map(|s| s.trim()) {
        if seq.contains("=") {
            let key = seq.split("=").next().unwrap().trim();
            let value = seq.split("=").nth(1).unwrap().parse::<usize>().unwrap();
            if let Some(seq) = boxes.iter_mut().find(|b| b.iter().any(|e| e.0 == key)) {
                if let Some(entry) = seq.iter_mut().find(|e| e.0 == key) {
                    entry.1 = value;
                }
            } else {
                let hash = aoc_hash(key);
                if let Some(vector) = boxes.get_mut(hash) {
                    vector.push((key.to_string(), value));
                } else {
                    boxes.insert(hash, vec![(key.to_string(), value)]);
                }
            }
        } else if seq.contains("-") {
            let key = seq.split("-").next().unwrap();
            for box_ in boxes.iter_mut() {
                if let Some(index) = box_.iter().position(|x| x.0 == key) {
                    box_.remove(index);
                }
            }
        }
    }

    boxes.into_iter().enumerate().fold(0, |acc, (index, box_)| {
        acc + if box_.is_empty() {
            0
        } else {
            (index + 1)
                * box_
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (index, c)| acc + ((index + 1) * c.1))
        }
    })
}

#[test]
fn check_part1() {
    let oracle = 1320;
    let input = std::fs::read_to_string("src/aoc2023/tests/day15").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 145;
    let input = std::fs::read_to_string("src/aoc2023/tests/day15").unwrap();
    assert_eq!(oracle, part_2(input));
}
