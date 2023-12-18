fn check_neighbors(map: &Vec<Vec<char>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    if current.1!=0 && "-LF".contains(map[current.0][current.1 -1]) {
        neighbors.push((current.0, current.1 -1));
    }
    else if current.1+1 < map[current.0].len() && "-J7".contains(map[current.0][current.1 +1]) {
        neighbors.push((current.0, current.1 +1));
    }
    else if current.0!=0 &&"|7F".contains(map[current.0 -1][current.1]) {
        neighbors.push((current.0 -1, current.1));
    }
    else if current.0+1 < map.len() && "|JL".contains(map[current.0 +1][current.1]) {
        neighbors.push((current.0 +1, current.1));
    }
    neighbors
}


fn check_visited(map: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let mut queue: Vec<(usize, usize)> = vec![start];
    let mut visited: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
    visited[start.0][start.1] = 0;
    while !queue.is_empty() {
        let current = queue.remove(0);
        let neighbors = check_neighbors(map, current);
        for neighbor in neighbors {
            if visited[neighbor.0][neighbor.1] == 0 {
                visited[neighbor.0][neighbor.1] = visited[current.0][current.1] + 1;
                queue.push(neighbor);
            }
        }
    }
    visited[start.0][start.1] = 1;
    visited
}

pub fn part_1(input: String) -> usize {
    let map:Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let start: (usize, usize) = map.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&cell| cell == 'S').map(|j| (i, j))
    }).unwrap_or((0, 0)); // Default value in case 'S' is not found
    let visited = check_visited(&map, start);
    *visited.iter().flatten().max().unwrap()
}

fn group_neighbors(points: Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut grouped_points: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; points.len()];

    for (i, _point) in points.iter().enumerate() {
        if visited[i] {
            continue;
        }
        let mut group: Vec<(usize, usize)> = Vec::new();
        let mut stack: Vec<usize> = vec![i];

        while let Some(idx) = stack.pop() {
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            group.push(points[idx]);

            for (j, other_point) in points.iter().enumerate() {
                if !visited[j] && is_neighbor(points[idx], *other_point) {
                    stack.push(j);
                }
            }
        }
        if !group.is_empty() {
            grouped_points.push(group);
        }
    }
    grouped_points
}

fn is_neighbor(p1: (usize, usize), p2: (usize, usize)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x1 == x2 && y2!=0 && (y1 == y2 + 1 || y1 == y2 - 1) || y1 == y2 && x2!=0&&(x1 == x2 + 1 || x1 == x2 - 1)
}

fn check_enclosed(map: &Vec<Vec<usize>>) -> usize {
    let zeros: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == 0 {  Some((i, j)) } else {  None }
        })
    }).collect();
    let zeros = group_neighbors(zeros);
    let zeros = zeros.iter().filter(|group| {
        group.iter().all(|point|
            point.0 != 0 && point.1 != 0 &&
            point.0 != map.len()-1 && point.1 != map[0].len()-1)
    });
    // for line in map.iter() {
    //     for e in line {
    //         print!("|{:2}", e);
    //     }
    //     println!("|");
    // }
    // for group in zeros.clone().into_iter() {
    //     println!("{:?}", group);
    // }
    // for (i, row) in map.iter().enumerate() {
    //     for (j, e) in row.iter().enumerate() {
    //         if *e == 0 {
    //             let mut found = false;
    //             for z in zeros.clone().into_iter() {
    //                 for p in z {
    //                     if p.0 == i && p.1 == j {
    //                         print!("I");
    //                         found = true;
    //                     }
    //                 }
    //             }
    //             if !found {
    //                 print!("O");
    //             }
    //         } else {
    //             print!("X");
    //         }
    //     }
    //     println!();
    // }
    zeros.into_iter().flatten().count()
}

pub fn part_2(input: String) -> usize {
    let map:Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let start: (usize, usize) = map.iter().enumerate().find_map(|(i, row)| {
        row.iter().position(|&cell| cell == 'S').map(|j| (i, j))
    }).unwrap_or((0, 0)); // Default value in case 'S' is not found
    let visited = check_visited(&map, start);
    // check_enclosed(&visited)
    0
}

#[test]
#[ignore]
fn check_part1_1(){
    let oracle = 4;
    let input = std::fs::read_to_string("src/aoc2023/tests/day10_1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
#[ignore]
fn check_part1_2(){
    let oracle = 8;
    let input = std::fs::read_to_string("src/aoc2023/tests/day10_2").unwrap();
    assert_eq!(oracle, part_1(input));
}


#[test]
#[ignore]
fn check_part2_1(){
    let oracle = 10;
    let input = std::fs::read_to_string("src/aoc2023/tests/day10_3").unwrap();
    assert_eq!(oracle, part_2(input));
}


#[test]
#[ignore]
fn check_part2_2(){
    let oracle = 8;
    let input = std::fs::read_to_string("src/aoc2023/tests/day10_4").unwrap();
    assert_eq!(oracle, part_2(input));
}
#[test]
#[ignore]
fn check_part2_3(){
    let oracle = 4;
    let input = std::fs::read_to_string("src/aoc2023/tests/day10_5").unwrap();
    assert_eq!(oracle, part_2(input));
}
