use std::usize;

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn count_quadrant(
    robots: &[Robot],
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> usize {
    let mut count = 0;
    for x in start_x..end_x {
        for y in start_y..end_y {
            count += robots
                .iter()
                .filter(|r| r.p.0 == x as i64 && r.p.1 == y as i64)
                .count();
        }
    }
    count
}

pub fn part_1(input: String) -> usize {
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(" ").unwrap();
            let p = p_str.split_once("=").unwrap().1.split_once(",").unwrap();
            let p = (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap());
            let v = v_str.split_once("=").unwrap().1.split_once(",").unwrap();
            let v = (v.0.parse::<i64>().unwrap(), v.1.parse::<i64>().unwrap());
            Robot { p, v }
        })
        .collect();
    let wide = 101;
    let tall = 103;
    for _s in 0..100 {
        for r in &mut robots {
            r.p = (
                (r.p.0 + r.v.0).rem_euclid(wide),
                (r.p.1 + r.v.1).rem_euclid(tall),
            );
        }
    }
    let wide = wide as usize;
    let tall = tall as usize;
    let q1 = count_quadrant(&robots, 0, 0, wide / 2, tall / 2);
    let q2 = count_quadrant(&robots, (wide / 2) + 1, 0, wide, tall / 2);
    let q3 = count_quadrant(&robots, 0, (tall / 2) + 1, wide / 2, tall);
    let q4 = count_quadrant(&robots, (wide / 2) + 1, (tall / 2) + 1, wide, tall);
    q1 * q2 * q3 * q4
}

pub fn part_2(input: String) -> usize {
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(" ").unwrap();
            let p = p_str.split_once("=").unwrap().1.split_once(",").unwrap();
            let p = (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap());
            let v = v_str.split_once("=").unwrap().1.split_once(",").unwrap();
            let v = (v.0.parse::<i64>().unwrap(), v.1.parse::<i64>().unwrap());
            Robot { p, v }
        })
        .collect();
    let wide = 101;
    let tall = 103;
    let mut time = 0;

    loop {
        let mut grid = vec![vec![false; wide]; tall];
        for r in &robots {
            let x = r.p.0 as usize;
            let y = r.p.1 as usize;
            grid[y][x] = true;
        }
        for y in 0..tall {
            let mut consecutive_count = 0;

            for x in 0..wide {
                if grid[y][x] {
                    consecutive_count += 1;
                } else {
                    consecutive_count = 0;
                }
                if consecutive_count >= 5 {
                    println!("DEBUGPRINT[7]: day14.rs:81: s={:#?}", time);
                    for i in 0..grid.len() {
                        for j in 0..grid[i].len() {
                            if grid[i][j] {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                        println!();
                    }
                }
            }
            for r in &mut robots {
                r.p = (
                    (r.p.0 + r.v.0).rem_euclid(wide as i64),
                    (r.p.1 + r.v.1).rem_euclid(tall as i64),
                );
            }
            time += 1;
        }
    }
    time
}

#[test]
fn check_part1() {
    let oracle = 480;
    let input = std::fs::read_to_string("src/aoc2024/tests/day14.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 1206;
    let input = std::fs::read_to_string("src/aoc2024/tests/day14.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
