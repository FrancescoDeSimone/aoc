use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x1, y1) = Self::extended_gcd(b, a % b);
            (g, y1, x1 - (a / b) * y1)
        }
    }

    fn diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
        let (g, x, y) = Self::extended_gcd(a, b);
        if c % g != 0 {
            return None;
        }
        let factor = c / g;
        Some((x * factor, y * factor))
    }

    fn calculate_min_tokens(&self) -> Option<i64> {
        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (px, py) = self.prize;

        let (a_x, b_x) = Self::diophantine(ax, bx, px)?;
        let (a_y, b_y) = Self::diophantine(ay, by, py)?;

        if a_x < 0 || b_x < 0 || a_y < 0 || b_y < 0 {
            return None;
        }
        let cost_x = a_x * 3 + b_x;
        let cost_y = a_y * 3 + b_y;
        Some(cost_x + cost_y)
    }
}

pub fn part_1(input: String) -> i64 {
    let regex = Regex::new(r"(?m)(\d+),.*?(\d+)").unwrap();
    input
        .split("\n\n")
        .map(|block| {
            let captures: Vec<(i64, i64)> = regex
                .captures_iter(block)
                .filter_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
                .collect();

            Machine {
                a: captures[0],
                b: captures[1],
                prize: captures[2],
            }
        })
        .map(|machine| {
            (0..100_i64)
                .flat_map(|i| (0..100_i64).map(move |j| (i, j)))
                .find(|&(i, j)| {
                    let (rx, ry) = (
                        (machine.b.0 * j) + (machine.a.0 * i),
                        (machine.b.1 * j) + (machine.a.1 * i),
                    );
                    rx == machine.prize.0 && ry == machine.prize.1
                })
                .map(|(i, j)| i * 3 + j)
                .unwrap_or(0)
        })
        .sum()
}

pub fn part_2(input: String) -> i64 {
    let regex = Regex::new(r"(?m)(\d+),.*?(\d+)").unwrap();
    //let offset = 10_000_000_000_000i64;
    let offset = 0;
    input
        .split("\n\n")
        .map(|block| {
            let captures: Vec<(i64, i64)> = regex
                .captures_iter(block)
                .filter_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
                .collect();

            Machine {
                a: captures[0],
                b: captures[1],
                prize: (captures[2].0 + offset, captures[2].1 + offset),
            }
        })
        .filter_map(|machine| machine.calculate_min_tokens())
        .sum()
}

#[test]
fn check_part1() {
    let oracle = 480;
    let input = std::fs::read_to_string("src/aoc2024/tests/day13.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 1206;
    let input = std::fs::read_to_string("src/aoc2024/tests/day13.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
