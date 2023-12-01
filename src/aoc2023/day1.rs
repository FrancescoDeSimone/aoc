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

fn matching_number(number: &str) -> Vec<u32> {
    let mut res = Vec::new();
    for start in 0..number.chars().count() {
        for end in (start+1)..number.chars().count().min(start+5) {
            let num = &number[start..end+1];
            res.push(match num {
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
            });
        }}
    res.into_iter().filter(|x| *x != 0).collect()
}

pub fn part_2(input: String) -> usize {

    let mut sum = 0;
    for lines in input.lines() {
        let l = lines.chars().collect::<Vec<char>>();
        for i in 0..lines.len() {
            if l[i].is_numeric(){
                let n = matching_number(&String::from_iter(&l[0..i]));
                if !n.is_empty(){
                    sum += 10*n.first().unwrap();
                    break;
                }else{
                    sum += 10*l[i].to_digit(10).unwrap();
                    break;
                }
            }
        }
        for i in (0..lines.len()).rev() {
            if l[i].is_numeric(){
                let n = matching_number(&String::from_iter(&l[(i+1)..l.len()]));
                if !n.is_empty(){
                sum += n.last().unwrap();
                    break;
                }else{
                    sum += l[i].to_digit(10).unwrap();
                    break;
                }
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
