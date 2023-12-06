fn parse_time_distance(input: String) -> (Vec<u64>, Vec<u64>) {
    let (time,distance) = input.split_once("\n").unwrap_or(("",""));
    let time = time.split_once(":").unwrap().1
        .split(" ").filter(|e| !e.is_empty())
        .map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let distance = distance.split_once(":").unwrap().1;
    let distance = distance.replace("\n", "");
    let distance = distance.split(" ").filter(|e| !e.is_empty()).map(|e| e.parse::<u64>()
        .unwrap()).collect::<Vec<u64>>();
    (time, distance)

}

fn get_winning(time: u64, distance: u64) -> u64 {
    (1..time).fold(0, |winning_time,t| {
        let time_left = time - t;
        if distance < (time_left * (t as u64)) { winning_time + 1 } else { winning_time }
    })
}

pub fn part_1(input: String) -> u64 {
    let mut win = Vec::new();
    let (time, distance) = parse_time_distance(input);
    for i in 0..time.len() {
        win.push(get_winning(time[i], distance[i]));
    }
    win.into_iter().filter(|e| *e != 0).reduce(|a,b| a*b).unwrap()
}

pub fn part_2(input: String) -> u64 {
    let (time, distance) = parse_time_distance(input);
    let time = time.into_iter()
        .map(|e| e.to_string())
        .reduce(|a,b| a+&b).unwrap().parse::<u64>().unwrap();

    let distance = distance.into_iter()
        .map(|e| e.to_string())
        .reduce(|a,b| a+&b).unwrap().parse::<u64>().unwrap();
    get_winning(time, distance)
}

#[test]
fn check_part1(){
    let oracle = 288;
    let input = std::fs::read_to_string("src/aoc2023/tests/day6").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 71503;
    let input = std::fs::read_to_string("src/aoc2023/tests/day6").unwrap();
    assert_eq!(oracle, part_2(input));
}
