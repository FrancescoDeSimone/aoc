pub fn part_1(input: String) -> u32 {
    let mut res = 0;
    for line in input.lines(){
        let game = line.split_once(":").unwrap();
        let winnning = game.1.split_once("|").unwrap().0;
        let numbers = game.1.split_once("|").unwrap().1;
        let winnning = winnning.trim().split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let numbers = numbers.trim().split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let numbers = numbers.into_iter().filter(|e| winnning.contains(e)).count();
        if numbers > 0 {
            res += (1..numbers).fold(1, |a, _| a * 2);
        }
    }
    res
}

pub fn part_2(input: String) -> u32 {
    let mut scratchcards = vec![1; input.lines().count()];
    for line in input.lines(){
        let game = line.split_once(":").unwrap();
        let id = game.0.split_once(" ").unwrap().1.trim().parse::<usize>().unwrap();
        let winnning = game.1.split_once("|").unwrap().0;
        let numbers = game.1.split_once("|").unwrap().1;
        let winnning = winnning.trim().split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let numbers = numbers.trim().split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let numbers = numbers.into_iter().filter(|e| winnning.contains(e)).count();
        let id = id - 1;
        for _ in 0..scratchcards[id]{
            for i in (id+1)..(id+1+numbers) {
                if i >= scratchcards.len() {
                    break;
                }
                scratchcards[i] +=1;
            }
        }
    }
    scratchcards.iter().sum()
}


#[test]
fn check_part1(){
    let oracle = 13;
    let input = std::fs::read_to_string("src/aoc2023/tests/day4.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 30;
    let input = std::fs::read_to_string("src/aoc2023/tests/day4.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
