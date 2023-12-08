use std::{cmp::Ordering, collections::HashMap};

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum Type {
    Fivekind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq)]
struct Set {
    cards: Vec<char>,
    type_card: Type,
    points: u64,
    order: Vec<char>
}

impl Set {
    fn calc_type(counter: &HashMap<char,u64>) -> Type {
        match counter.keys().len() {
            1 => Type::Fivekind,
            2 => {
                let v = *counter.values().collect::<Vec<&u64>>()[0];
                if v == 4 || v == 1 { Type::FourKind } else { Type::FullHouse }
            }
            3 => {
                let v = *counter.values().max().unwrap();
                if v == 3 { Type::ThreeKind } else { Type::TwoPair }
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unimplemented!()
        }
    }

    fn new(set: &str, points: &str, order:Vec<char>) -> Set {
        let mut cards = Vec::new();
        let mut counter = HashMap::new();

        for c in set.chars() {
            cards.push(c);
            counter.entry(c).and_modify(|count| *count +=1).or_insert(1);
        }

        if order.last().unwrap() == &'J' {
            if counter.len() > 1 && counter.contains_key(&'J')  {
                if let Some(max_char) = counter.iter().filter(|e| e.0 != &'J').max_by(|a, b| a.1.cmp(&b.1)){
                    if let Some(&j_count) = counter.get(&'J') {
                        counter.entry(*max_char.0).and_modify(|count| *count += j_count);
                        counter.remove(&'J');
                    }
                }
            }
        }

        Set {
            cards,
            type_card: Self::calc_type(&counter),
            points: points.parse().unwrap(),
            order
        }
    }
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.type_card == other.type_card && self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.type_card < other.type_card {
            return Ordering::Greater;
        } else if self.type_card == other.type_card {
            let order = self.order.clone()
                .into_iter().enumerate()
                .map(|(i,c)| (c,i)).collect::<HashMap<char,usize>>();

            for i in 0..self.cards.len() {
                if order.get(&self.cards[i]).unwrap() < order.get(&other.cards[i]).unwrap() {
                    return Ordering::Greater;
                } else if order.get(&self.cards[i]).unwrap() > order.get(&other.cards[i]).unwrap() {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }else {
            return Ordering::Less;
        }
    }
}

pub fn part_1(input: String) -> u64 {
    let mut sets:Vec<Set> = Vec::new();
    for line in input.lines() {
        let (set,points) = line.split_once(" ").unwrap();
        let order = vec!['A','K','Q','J','T','9','8','7','6','5','4','3','2'];
        let set = Set::new(set,points,order);
        sets.push(set);
    }
    sets.sort();
    sets.into_iter().enumerate().map(|(i,s)| (i+1,s)).fold(0, |acc, (i,s)| acc + (s.points * i as u64))
}

pub fn part_2(input: String) -> u64 {
    let mut sets:Vec<Set> = Vec::new();
    for line in input.lines() {
        let (set,points) = line.split_once(" ").unwrap();
        let order = vec!['A','K','Q','T','9','8','7','6','5','4','3','2','J'];
        let set = Set::new(set,points,order);
        sets.push(set);
    }
    sets.sort();
    sets.into_iter().enumerate().map(|(i,s)| (i+1,s)).fold(0, |acc, (i,s)| acc + (s.points * i as u64))
}

#[test]
fn check_part1(){
    let oracle = 6440;
    let input = std::fs::read_to_string("src/aoc2023/tests/day7").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2(){
    let oracle = 5905;
    let input = std::fs::read_to_string("src/aoc2023/tests/day7").unwrap();
    assert_eq!(oracle, part_2(input));
}
