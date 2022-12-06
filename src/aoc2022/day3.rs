use regex::Regex;

pub fn part_1(input: String) -> usize {
    let mut priority: usize = 0;
    for rucksacks in input.split('\n').filter(|x| !x.is_empty()) {
        let (sack_l, sack_r) = rucksacks.split_at(rucksacks.len() / 2);
        for e in sack_l.chars() {
            if sack_r.contains(e) {
                if e.is_lowercase() {
                    priority += 1 + (e as usize - 'a' as usize);
                } else {
                    priority += 27 + (e as usize - 'A' as usize);
                }
                break;
            }
        }
    }
    priority
}
pub fn part_2(input: String) -> usize {
    let mut priority: usize = 0;
    let group_regex = Regex::new(r"(?m).*?\n.*?\n.*?\n").unwrap();
    for group in group_regex.captures_iter(&input) {
        let rucksacks = group
            .get(0)
            .unwrap()
            .as_str()
            .split('\n')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        for c in rucksacks[0].chars() {
            if rucksacks[1].contains(c) && rucksacks[2].contains(c) {
                if c.is_lowercase() {
                    priority += 1 + (c as usize - 'a' as usize);
                } else {
                    priority += 27 + (c as usize - 'A' as usize);
                }
                break;
            }
        }
    }
    priority
}
