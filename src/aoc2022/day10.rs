use std::collections::VecDeque;

pub fn part_1(input: String) -> isize {
    let mut cycle = 0;
    let mut op_queue = VecDeque::new();
    for ops in input.split('\n').filter(|e| !e.is_empty()) {
        let (op, value) = ops.split_at(ops.find(" ").unwrap_or(ops.len()));
        match op {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 2;
                op_queue.push_back((cycle, value.trim().parse::<isize>().unwrap()));
            }
            _ => unreachable!(),
        };
    }

    let mut x = 1;
    let mut res = Vec::new();
    for cycle in 0..=220 {
        res.push(match cycle {
            20 => cycle * x,
            60 => cycle * x,
            100 => cycle * x,
            140 => cycle * x,
            180 => cycle * x,
            220 => cycle * x,
            _ => 0,
        });

        if cycle == op_queue.front().unwrap().0 {
            let op = op_queue.pop_front().unwrap();
            x += op.1;
        };
    }
    res.into_iter().sum()
}

pub fn part_2(input: String) -> String {
    let mut cycle = 0;
    let mut op_queue = VecDeque::new();
    for ops in input.split('\n').filter(|e| !e.is_empty()) {
        let (op, value) = ops.split_at(ops.find(" ").unwrap_or(ops.len()));
        match op {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 2;
                op_queue.push_back((cycle, value.trim().parse::<isize>().unwrap()));
            }
            _ => unreachable!(),
        };
    }

    let mut display = String::from("\n");
    let mut cycle = 0;
    let mut x = 1;
    for _ in 0..6 {
        for line in 1..=40 {
            if op_queue.front().is_some() && cycle == op_queue.front().unwrap().0 {
                let op = op_queue.pop_front().unwrap();
                x += op.1;
            };
            if line == x || line == x + 1 || line == x + 2 {
                display += "#";
            } else {
                display += ".";
            }
            cycle += 1;
        }
        display += "\n";
    }
    display
}

#[test]
fn check_part2() {
    let oracle = "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part_2(input.to_string()), oracle);
}
