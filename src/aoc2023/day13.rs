fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn check_horizontal(map: &[Vec<char>]) -> u64 {
    if let Some(reflection_index) = (0..map.len() - 1).find(|&i| map[i] == map[i + 1]) {
        let (mut back, mut forth) = (reflection_index as i64, reflection_index + 1);
        while back >= 0 && forth < map.len() {
            println!(
                "comparing {} {}\n{:?}\n{:?}",
                back + 1,
                forth + 1,
                map[back as usize],
                map[forth]
            );
            if map[back as usize] != map[forth] {
                return 0;
            }
            back -= 1;
            forth += 1;
        }
        return (reflection_index + 1) as u64;
    }
    0
}

fn print_debug(map: &[Vec<char>]) {
    println!("------------");
    for (index, row) in map.iter().enumerate() {
        print!("{} ", index + 1);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("------------");
}

fn check_vertical(map: &Vec<Vec<char>>) -> u64 {
    let mut i = 0;
    let mut j = 0;
    let mut reflection_index = -1;
    while i < map.len() - 1 {
        let mut is_mirror = true;
        while j < map[i].len() && is_mirror {
            if map[i][j] != map[i][j + 1] {
                is_mirror = false;
            }
            j += 1;
        }
        if is_mirror {
            reflection_index = j as i64;
            break;
        }
        i += 1;
    }
    if reflection_index == -1 {
        return 0;
    }
    let (mut back, mut forth) = (reflection_index as i64, (reflection_index + 1) as usize);
    while back >= 0 && forth < map.len() {
        println!(
            "comparing {} {}\n{:?}\n{:?}",
            back + 1,
            forth + 1,
            map[back as usize],
            map[forth as usize]
        );
        if map[back as usize] != map[forth] {
            return 0;
        }
        back -= 1;
        forth += 1;
    }
    reflection_index as u64
}

pub fn part_1(input: String) -> u64 {
    let mut res = 0;
    for terrain in input.split("\n\n") {
        let map: Vec<Vec<char>> = terrain.lines().map(|l| l.chars().collect()).collect();
        let vertical = check_vertical(&map);
        println!("vertical: {}", vertical);
        let horizontal = check_horizontal(&map) * 100;
        println!("horizontal: {}", horizontal);
        res += vertical + horizontal;
    }
    res
}

pub fn part_2(input: String) -> u64 {
    0
}

#[test]
fn check_part1() {
    let oracle = 405;
    let input = std::fs::read_to_string("src/aoc2023/tests/day13").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part1_2() {
    let oracle = 0;
    let input = std::fs::read_to_string("src/aoc2023/tests/day13_2").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 0;
    let input = std::fs::read_to_string("src/aoc2023/tests/day13").unwrap();
    assert_eq!(oracle, part_2(input));
}
