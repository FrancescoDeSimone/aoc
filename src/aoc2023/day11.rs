fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_galaxies(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == '#' { Some((i, j)) } else { None }
        })
    }).collect()
}

fn extend_galaxy(galaxies: &Vec<(usize, usize)>, distance: usize) -> Vec<(usize, usize)> {
    let mut real_galaxy = vec![];
    for &(x,y) in galaxies {
        let x = (0..x).fold(x, |acc, index|
            acc + if galaxies.iter().all(|(x, _)| *x != index ) { distance -1 }
            else {0});

        let y = (0..y).fold(y, |acc, index|
            acc + if galaxies.iter().all(|(_, y)| *y != index ) { distance -1 }
            else {0});
        real_galaxy.push((x, y));
    }
    real_galaxy
}

pub fn part_1(input: String) -> i64 {
    let map = parse_input(&input);
    let galaxies = find_galaxies(&map);
    let galaxies= extend_galaxy(&galaxies, 2);

    galaxies.iter().enumerate().flat_map(|(i, start)| {
        galaxies.iter().skip(i + 1).map(move |goal| {
            let dx = (start.0 as i64).checked_sub(goal.0 as i64).unwrap_or(i64::MAX);
            let dy = (start.1 as i64).checked_sub(goal.1 as i64).unwrap_or(i64::MAX);
            dx.abs().saturating_add(dy.abs())
        })
    }).sum()
}

pub fn part_2(input: String) -> i64 {
    let map = parse_input(&input);
    let galaxies = find_galaxies(&map);
    let galaxies = extend_galaxy(&galaxies, 1000000);

    galaxies.iter().enumerate().flat_map(|(i, start)| {
        galaxies.iter().skip(i + 1).map(move |goal| {
            let dx = (start.0 as i64).checked_sub(goal.0 as i64).unwrap_or(i64::MAX);
            let dy = (start.1 as i64).checked_sub(goal.1 as i64).unwrap_or(i64::MAX);
            dx.abs().saturating_add(dy.abs())
        })
    }).sum()

}

#[test]
fn check_part1(){
    let oracle = 374;
    let input = std::fs::read_to_string("src/aoc2023/tests/day11").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 82000210;
    let input = std::fs::read_to_string("src/aoc2023/tests/day11").unwrap();
    assert_eq!(oracle, part_2(input));
}
