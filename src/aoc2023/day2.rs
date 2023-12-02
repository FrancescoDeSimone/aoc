fn extract_balls(set: &Vec<&str>) -> [u32;3]{
    let mut balls:[u32;3] = [0,0,0];
    for ball in set {
        let ball = ball.trim();
        let ball = ball.split(" ").collect::<Vec<&str>>();
        match ball[1] {
            "red" => balls[0] += ball[0].parse::<u32>().unwrap(),
            "green" => balls[1] += ball[0].parse::<u32>().unwrap(),
            "blue" => balls[2] += ball[0].parse::<u32>().unwrap(),
            _ => unreachable!()
        };
    }
    balls
}

pub fn check_possible_sets(sets: &Vec<&str>) -> bool {
    for set in sets{
        let set = set.split(",").collect::<Vec<&str>>();
        let balls:[u32;3] = extract_balls(&set);
        if !(balls[0] <= 12 && balls[1] <= 13 && balls[2] <= 14) { return false;}
    }
    true
}

pub fn part_1(input: String) -> usize {
    let mut res = 0;
    for game in input.lines() {
        let game = game.split(":").collect::<Vec<&str>>();
        let id = game[0].split(" ").last().unwrap().parse::<usize>().unwrap();
        if game.last().is_some(){
            let sets = game.last().unwrap().split(";").collect::<Vec<&str>>();
            if check_possible_sets(&sets) {res += id;}
        }
    }
    res
}

pub fn part_2(input: String) -> usize {
    let mut res = 0;
    for game in input.lines() {
        let game = game.split(":").collect::<Vec<&str>>();
        if game.last().is_some(){
            let sets = game.last().unwrap().split(";").collect::<Vec<&str>>();
            let mut power_balls:[u32;3] = [0,0,0];
            for set in sets{
                let set = set.split(",").collect::<Vec<&str>>();
                let balls:[u32;3] = extract_balls(&set);
                power_balls[0] = power_balls[0].max(balls[0]);
                power_balls[1] = power_balls[1].max(balls[1]);
                power_balls[2] = power_balls[2].max(balls[2]);
            }
            res += power_balls[0] * power_balls[1] * power_balls[2];
        }
    }
    res as usize
}


#[test]
fn check_part1(){
    let oracle = 8;
    let input = std::fs::read_to_string("src/aoc2023/tests/day2.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 2286;
    let input = std::fs::read_to_string("src/aoc2023/tests/day2.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
