pub fn part_1(input: String) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        let mut history: Vec<Vec<i64>> = vec![];
        history.push(line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>());
        loop {
            let current = history.last().unwrap();
            if current.iter().all(|x| *x == 0) {
                break;
            }
            history.push(
                (1..current.len()).
                    fold(vec![], |mut acc, index|
                        { acc.push(current[index] - current[index-1]);acc})
            );
        };
        for i in (1..history.len()).rev(){
            let current = *history[i].last().unwrap_or(&0);
            let prev = *history[i-1].last().unwrap_or(&0);
            history[i-1].push(current + prev);
        }
        res += history.first().unwrap().last().unwrap();
    }
    res
}

pub fn part_2(input: String) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        let mut history: Vec<Vec<i64>> = vec![];
        history.push(line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>());
        loop {
            let current = history.last().unwrap();
            if current.iter().all(|x| *x == 0) {
                break;
            }
            history.push(
                (1..current.len()).
                    fold(vec![], |mut acc, index|
                        { acc.push(current[index] - current[index-1]);acc})
            );
        };
        for i in (1..history.len()).rev(){
            let current = *history[i].first().unwrap_or(&0);
            let prev = *history[i-1].first().unwrap_or(&0);
            history[i-1].insert(0,-current + prev);
        }
        res += history.first().unwrap().first().unwrap();
    }
    res
}

#[test]
fn check_part1(){
    let oracle = 114;
    let input = std::fs::read_to_string("src/aoc2023/tests/day9").unwrap();
    assert_eq!(oracle, part_1(input));
}


#[test]
fn check_part2(){
    let oracle = 2;
    let input = std::fs::read_to_string("src/aoc2023/tests/day9").unwrap();
    assert_eq!(oracle, part_2(input));
}
