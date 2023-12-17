fn extend_galaxy(galaxies: Vec<(usize, usize)>, distance: usize) -> Vec<(usize, usize)> {
    let mut real_galaxy = vec![];
    for (_, galaxy) in galaxies.iter().enumerate() {
        let x = (0..galaxy.0).fold(galaxy.0, |acc, index|
            acc + if galaxies.iter().all(|(x, _)| *x != index ) { distance -1 }
            else {0});

        let y = (0..galaxy.1).fold(galaxy.1, |acc, index|
            acc + if galaxies.iter().all(|(_, y)| *y != index ) { distance -1 }
            else {0});
        real_galaxy.push((x, y));
    }
    real_galaxy
}

pub fn part_1(input: String) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let galaxies: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == '#' {  Some((i, j)) } else {  None }
        })
    }).collect();
    let galaxies: Vec<(usize, usize)> = extend_galaxy(galaxies, 2);

    galaxies.iter().enumerate().flat_map(|(i, start)| {
        galaxies.iter().skip(i + 1).map(move |goal| {
            let dx = (start.0 as i64).checked_sub(goal.0 as i64).unwrap_or(i64::MAX);
            let dy = (start.1 as i64).checked_sub(goal.1 as i64).unwrap_or(i64::MAX);
            dx.abs().saturating_add(dy.abs())
        })
    }).sum()
}

pub fn part_2(input: String) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();
    let galaxies: Vec<(usize, usize)> = map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &cell)| {
            if cell == '#' {  Some((i, j)) } else {  None }
        })
    }).collect();
    let galaxies: Vec<(usize, usize)> = extend_galaxy(galaxies, 1000000);

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
