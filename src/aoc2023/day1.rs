pub fn part_1(input: String) -> usize {
    let mut sum = 0;
    for lines in input.lines() {
        let l = lines.chars().collect::<Vec<char>>();
        for i in 0..lines.len() {
            if l[i].is_numeric(){
                sum += 10*l[i].to_digit(10).unwrap();
                break;
            }
        }

        for i in (0..lines.len()).rev() {
            if l[i].is_numeric(){
                sum += l[i].to_digit(10).unwrap();
                break;
            }
        }
    }
    sum as usize
}

fn matching_number(number: &str) -> u32 {
    for start in 0..number.chars().count() {
        for end in (start+1)..number.chars().count() {
            let num = &number[start..end+1];
            let n = match num {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0
            };
            if n > 0 {
                return n;
            }
        }}
    0
}

pub fn part_2(input: String) -> usize {

    let mut sum = 0;
    for lines in input.lines() {
        let l = lines.chars().collect::<Vec<char>>();
        let mut number = String::new();
        for i in 0..lines.len() {
            number += &l[i].to_string();
            let n = matching_number(&number);
            if n > 0{
                sum += 10*n;
                break;
            }

            if l[i].is_numeric(){
                sum += 10*l[i].to_digit(10).unwrap();
                break;
            }
        }

        let mut number = String::new();
        for i in (0..lines.len()).rev() {

            number = l[i].to_string() + &number;
            let n = matching_number(&number);
            if n > 0{
                sum += n;
                break;
            }
            if l[i].is_numeric(){
                sum += l[i].to_digit(10).unwrap();
                break;
            }
        }
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
