use regex::Regex;

pub fn part_1(input: String) -> String {
    let mut space = 0;
    let mut index = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(Vec::new());
    for char in input
        .split("\n\n")
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .chars()
    {
        if char == '[' || char == ']' {
            if space != 0 {
                space = 0;
            }
        } else if char == ' ' {
            if space == 0 {
                index += 1;
                stacks.push(Vec::new());
            }
            space += 1;
            space %= 4;
        } else if char == '\n' {
            index = 0;
        } else if char.is_alphabetic() {
            stacks[index].push(char);
        }
    }
    stacks = stacks
        .into_iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.into_iter().rev().collect())
        .collect();
    let regex = Regex::new(r"(?m)^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    let movement = regex.captures_iter(&input);
    for m in movement {
        let quantity = m.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let from = m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = m.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let l = stacks[from].len();
        let mut moving = stacks[from]
            .drain(l - quantity - 1..l)
            .collect::<Vec<char>>();
        moving.reverse();
        for e in moving {
            stacks[to].push(e);
        }
    }
    stacks
        .into_iter()
        .map(|mut e: Vec<char>| e.pop().unwrap_or(' ').to_string())
        .collect::<String>()
}

pub fn part_2(input: String) -> String {
    let mut space = 0;
    let mut index = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.push(Vec::new());
    for char in input
        .split("\n\n")
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .chars()
    {
        if char == '[' || char == ']' {
            if space != 0 {
                space = 0;
            }
        } else if char == ' ' {
            if space == 0 {
                index += 1;
                stacks.push(Vec::new());
            }
            space += 1;
            space %= 4;
        } else if char == '\n' {
            index = 0;
        } else if char.is_alphabetic() {
            stacks[index].push(char);
        }
    }
    stacks = stacks
        .into_iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.into_iter().rev().collect())
        .collect();
    let regex = Regex::new(r"(?m)^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    let movement = regex.captures_iter(&input);
    for m in movement {
        let quantity = m.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let from = m.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = m.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let l = stacks[from].len();
        let moving = stacks[from]
            .drain(l - quantity - 1..l)
            .collect::<Vec<char>>();
        for e in moving {
            stacks[to].push(e);
        }
    }
    stacks
        .into_iter()
        .map(|mut e: Vec<char>| e.pop().unwrap_or(' ').to_string())
        .collect::<String>()
}
