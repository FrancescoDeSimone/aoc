pub fn part_1(input: String) -> usize {
    let mut sum = 0;
    for lines in input.lines() {
        let input = lines.chars().collect::<Vec<char>>();

        let index = lines.find(|x: char| x.is_numeric()).unwrap();
        sum += 10 * input[index].to_digit(10).unwrap();

        let index = lines.len() - lines.chars().rev().position(|x: char| x.is_numeric()).unwrap() - 1;
        sum += input[index].to_digit(10).unwrap();
    }
    sum as usize
}

fn matching_number(number: &str) -> Vec<u32> {
    let mut res = Vec::new();
    for start in 0..number.len() {
        for end in start+1..number.len().min(start+5) {
            let num = &number[start..end+1];
            if let Some(val) = match num {
                "one" => Some(1),
                "two" => Some(2),
                "three" => Some(3),
                "four" => Some(4),
                "five" => Some(5),
                "six" => Some(6),
                "seven" => Some(7),
                "eight" => Some(8),
                "nine" => Some(9),
                _ => None
            }{res.push(val);}
        }
    }
    res
}

pub fn part_2(input: String) -> usize {
    let mut sum = 0;
    for lines in input.lines() {
        let input = lines.chars().collect::<Vec<char>>();

        let index = lines.find(|x: char| x.is_numeric()).unwrap_or(lines.len());
        let number = matching_number(&lines[0..index]);
        sum += 10 * if number.is_empty() {   input[index].to_digit(10).unwrap() } else { *number.first().unwrap() };

        let index = lines.len() - lines.chars().rev().position(|x: char| x.is_numeric()).unwrap_or(lines.len()) - 1;
        let number = matching_number(&lines[index+1..lines.len()]);
        sum += if number.is_empty() {   input[index].to_digit(10).unwrap() } else { *number.last().unwrap() };
    }
    sum as usize
}


#[test]
fn check_part1(){
    let oracle =142;
    let input = std::fs::read_to_string("src/aoc2023/tests/day1.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle =281;
    let input = std::fs::read_to_string("src/aoc2023/tests/day1.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
