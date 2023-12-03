fn get_number(engine_schematic: &mut Vec<Vec<char>>,i:usize,j:usize) -> Option<u32> {
    let mut num = String::new();
    let mut start = j;
    while start >= 1 && engine_schematic[i][start].is_digit(10) {
        if !engine_schematic[i][start-1].is_digit(10) {
            break;
        }
        start -=1;
    }

    while start < engine_schematic[i].len() && engine_schematic[i][start].is_digit(10)  {
        num += &engine_schematic[i][start].to_string();
        engine_schematic[i][start] = '.';
        start +=1;
    }
    if let Ok(num) = num.parse::<u32>() {
        Some(num)
    } else {
        None
    }
}

fn  get_adjacent_numbers(engine_schematic: &mut Vec<Vec<char>>,i:usize,j:usize) -> Vec<u32> {
    let mut res = Vec::new();
    if i > 0 {
        if !engine_schematic[i-1][j].is_numeric() {
            res.push(get_number(engine_schematic,i-1,j-1));
            res.push(get_number(engine_schematic,i-1,j+1));
        } else {
            res.push(get_number(engine_schematic,i-1,j));
        }
    }
    if i < engine_schematic.len()-1{
        if !engine_schematic[i+1][j].is_numeric() {
            res.push(get_number(engine_schematic,i+1,j-1));
            res.push(get_number(engine_schematic,i+1,j+1));
        } else {
            res.push(get_number(engine_schematic,i+1,j));
        }
    }
    res.push(get_number(engine_schematic,i,j-1));
    res.push(get_number(engine_schematic,i,j+1));
    res.into_iter().filter(|e| e.is_some()).map(|e| e.unwrap()).collect::<Vec<u32>>()
}

pub fn part_1(input: String) -> u32 {
    let mut res: u32 = 0;
    let mut engine_schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for i in 0..engine_schematic.len() {
        for j in 0..engine_schematic[i].len() {
            if engine_schematic[i][j] != '.' && !engine_schematic[i][j].is_numeric() {
                let numbers = get_adjacent_numbers(&mut engine_schematic,i,j);
                res += numbers.iter().sum::<u32>();
            }
        }

    }
    res
}

pub fn part_2(input: String) -> u32 {
    let mut res: u32 = 0;
    let mut engine_schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..engine_schematic.len() {
        for j in 0..engine_schematic[i].len() {
            if engine_schematic[i][j] == '*' {
                let numbers = get_adjacent_numbers(&mut engine_schematic,i,j);
                if numbers.len() >  1 {
                    let number = numbers.into_iter().reduce(|a,b| a*b).unwrap();
                    res += number;
                }
            }
        }
    }
    res
}


#[test]
fn check_part1(){
    let oracle = 4361;
    let input = std::fs::read_to_string("src/aoc2023/tests/day3.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 467835;
    let input = std::fs::read_to_string("src/aoc2023/tests/day3.part1").unwrap();
    assert_eq!(oracle, part_2(input));
}
