fn expand(map: &Vec<Vec<char>>, galaxy: &Vec<(usize,usize)>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = Vec::new();
    for i in 0..map.len() {
        if galaxy.iter().any(|(x, _)| {
            *x == i
        }) {
            expanded.push(map[i].clone());
        } else {
            expanded.push(vec!['.'; map[i].len()]);
            expanded.push(vec!['.'; map[i].len()]);
        }
    }
    for i in 0..expanded.len() {
        let mut increase = 0;
        for j in 0..expanded[i].len() {
            if galaxy.iter().all(|(_, y)| {
                *y != j
            }) {
                expanded[i].insert(j+increase, '.');
                increase += 1;
            }
        }
    }
    expanded
}


pub fn part_1(input: String) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let galaxies: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == '#' {  Some((i, j)) } else {  None }
        })
    }).collect();
    let map = expand(&map, &galaxies);
    let galaxies: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == '#' {  Some((i, j)) } else {  None }
        })
    }).collect();

    let mut res = 0;
    for (i,start) in galaxies.iter().enumerate() {
        for j in i+1..galaxies.len() {
            let goal = galaxies[j];
            let d = (start.0 as i32 - goal.0 as i32).abs() + (start.1 as i32 - goal.1 as i32).abs();
            res +=d;
        }
    }

    res
}

pub fn part_2(input: String) -> usize {
    0
}

#[test]
fn check_part1(){
    let oracle = 374;
    let input = std::fs::read_to_string("src/aoc2023/tests/day11").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 4;
    let input = std::fs::read_to_string("src/aoc2023/tests/day11").unwrap();
    assert_eq!(oracle, part_2(input));
}
