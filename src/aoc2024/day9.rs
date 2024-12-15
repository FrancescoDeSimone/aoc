pub fn part_1(input: String) -> usize {
    let mut disk = input
        .trim_end()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(index, chunk)| {
            let count = (chunk[0] - b'0') as usize;
            let free = chunk.get(1).map_or(0, |&c| (c - b'0') as usize);
            (vec![index; count], free)
        })
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut j = disk.len() - 1;
    while i < j {
        while disk[i].1 > 0 {
            if disk[j].0.is_empty() {
                j -= 1;
            } else {
                let item = disk[j].0.pop().unwrap();
                disk[i].0.push(item);
                disk[i].1 -= 1;
            }
        }
        i += 1;
    }
    disk.iter()
        .flat_map(|(blocks, _)| blocks)
        .enumerate()
        .map(|(index, block)| index * block)
        .sum()
}

pub fn part_2(input: String) -> usize {
    let mut disk = input
        .trim_end()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(index, chunk)| {
            let count = (chunk[0] - b'0') as usize;
            let free = chunk.get(1).map_or(0, |&c| (c - b'0') as usize);
            (vec![index; count], free, Vec::new())
        })
        .collect::<Vec<_>>();

    for index in (0..disk.len()).rev() {
        let size = disk[index].0.len();
        for i in 0..index {
            if disk[i].1 >= size {
                let moved_blocks = disk[index].0.drain(..).collect::<Vec<_>>();
                disk[i].2.extend(moved_blocks);
                disk[i].1 -= size;
                if !disk[index].2.is_empty() {
                    disk[index].0.resize(size, 0);
                } else {
                    disk[index].1 += size;
                }
                break;
            }
        }
    }

    disk.iter()
        .flat_map(|(blocks, free, moved)| {
            blocks
                .iter()
                .chain(moved.iter())
                .chain(std::iter::repeat(&0).take(*free))
                .cloned()
        })
        .enumerate()
        .map(|(index, block)| index * block)
        .sum()
}

#[test]
fn check_part1() {
    let oracle = 1928;
    let input = std::fs::read_to_string("src/aoc2024/tests/day9.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 2858;
    let input = std::fs::read_to_string("src/aoc2024/tests/day9.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
