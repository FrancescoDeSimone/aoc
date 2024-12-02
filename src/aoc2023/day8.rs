use std::collections::HashMap;

pub fn part_1(input: String) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let dir = lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for i in 2..lines.len() {
        let (loc, fork) = lines[i].split_once(" = ").unwrap();
        let loc = loc.trim().to_string();
        let fork = fork
            .chars()
            .filter(|c| c != &'(' && c != &')')
            .collect::<String>();
        let fork = fork.split_once(", ").unwrap();
        map.insert(loc, (fork.0.to_string(), fork.1.to_string()));
    }

    let mut point = "AAA";
    let mut count = 0;
    loop {
        if point == "ZZZ" {
            break;
        }
        let index = (count as usize) % dir.len();
        count += 1;
        if dir[index] == 'R' {
            point = map.get(point).unwrap().1.as_str();
        } else {
            point = map.get(point).unwrap().0.as_str();
        }
    }
    count
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_2(input: String) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let dir = lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for i in 2..lines.len() {
        let (loc, fork) = lines[i].split_once(" = ").unwrap();
        let loc = loc.trim().to_string();
        let fork = fork
            .chars()
            .filter(|c| c != &'(' && c != &')')
            .collect::<String>();
        let fork = fork.split_once(", ").unwrap();
        map.insert(loc, (fork.0.to_string(), fork.1.to_string()));
    }
    let start_points = map
        .keys()
        .filter(|k| k.contains("A"))
        .collect::<Vec<&String>>();
    let mut vec_count = vec![];
    for mut point in start_points {
        let mut count = 0;
        loop {
            if point.contains('Z') {
                break;
            }
            let index = (count as usize) % dir.len();
            count += 1;
            if dir[index] == 'R' {
                point = &map.get(point).unwrap().1;
            } else {
                point = &map.get(point).unwrap().0;
            }
        }
        vec_count.push(count);
    }

    vec_count.iter().fold(1, |acc, x| lcm(acc, *x))
}

#[test]
fn check_part1() {
    let oracle = 2;
    let input = std::fs::read_to_string("src/aoc2023/tests/day8").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part1_2() {
    let oracle = 6;
    let input = std::fs::read_to_string("src/aoc2023/tests/day8_2").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 6;
    let input = std::fs::read_to_string("src/aoc2023/tests/day8.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
