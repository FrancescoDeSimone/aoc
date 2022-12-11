use regex::Regex;
use std::collections::{BTreeSet, HashMap};

pub fn part_1(input: String) -> usize {
    let mut current_path = String::new();
    let mut size_table: HashMap<String, usize> = HashMap::new();

    let cd_regex = Regex::new(r"\$ cd [a-zA-Z/.]+").unwrap();
    let file_regex = Regex::new("^[0-9]+ [a-zA-Z.]+$").unwrap();

    for line in input.split('\n').filter(|e| !e.is_empty()) {
        match line {
            line if cd_regex.is_match(line) => {
                let filename = line.split(' ').into_iter().last().unwrap();
                current_path = match filename {
                    "/" => "/".to_string(),
                    ".." => current_path
                        .split('/')
                        .into_iter()
                        .take(current_path.split("/").count() - 2)
                        .map(|e| e.to_string() + "/")
                        .collect::<String>(),
                    _ => current_path + filename + "/",
                }
            }
            line if file_regex.is_match(line) => {
                let size: usize = line
                    .split(' ')
                    .into_iter()
                    .take(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let current_size = size_table.get("/").unwrap_or(&0);
                size_table.insert("/".to_string(), current_size + size);
                current_path
                    .split('/')
                    .into_iter()
                    .map(|e| e.to_string())
                    .filter(|e| !e.is_empty())
                    .fold(String::new(), |all_path: String, folder| {
                        let root = if all_path.is_empty() {
                            String::from("/") + &folder
                        } else {
                            all_path.clone() + &folder + "/"
                        };
                        let current_size = size_table.get(&root.clone()).unwrap_or(&0);
                        size_table.insert(root.clone(), current_size + size);
                        String::from(root)
                    });
            }
            _ => {}
        }
    }
    size_table
        .clone()
        .into_iter()
        .filter(|e| e.1 < 100000)
        .map(|e| e.1)
        .sum::<usize>()
}

pub fn part_2(input: String) -> usize {
    let mut current_path = String::new();
    let mut size_table: HashMap<String, usize> = HashMap::new();

    let cd_regex = Regex::new(r"\$ cd [a-zA-Z/.]+").unwrap();
    let file_regex = Regex::new("^[0-9]+ [a-zA-Z.]+$").unwrap();

    for line in input.split('\n').filter(|e| !e.is_empty()) {
        match line {
            line if cd_regex.is_match(line) => {
                let filename = line.split(' ').into_iter().last().unwrap();
                current_path = match filename {
                    "/" => "/".to_string(),
                    ".." => current_path
                        .split('/')
                        .into_iter()
                        .take(current_path.split("/").count() - 2)
                        .map(|e| e.to_string() + "/")
                        .collect::<String>(),
                    _ => current_path + filename + "/",
                }
            }
            line if file_regex.is_match(line) => {
                let size: usize = line
                    .split(' ')
                    .into_iter()
                    .take(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let current_size = size_table.get("/").unwrap_or(&0);
                size_table.insert("/".to_string(), current_size + size);
                current_path
                    .split('/')
                    .into_iter()
                    .map(|e| e.to_string())
                    .filter(|e| !e.is_empty())
                    .fold(String::new(), |all_path: String, folder| {
                        let root = if all_path.is_empty() {
                            String::from("/") + &folder
                        } else {
                            all_path.clone() + &folder + "/"
                        };
                        let current_size = size_table.get(&root.clone()).unwrap_or(&0);
                        size_table.insert(root.clone(), current_size + size);
                        String::from(root)
                    });
            }
            _ => {}
        }
    }
    let disk_space = 70000000;
    let disk_occupied = size_table.get("/").unwrap();
    let needed = 30000000 - (disk_space - disk_occupied);
    size_table
        .into_iter()
        .map(|e| e.1)
        .collect::<BTreeSet<usize>>()
        .into_iter()
        .rev()
        .reduce(|res, size| if size >= needed { size } else { res })
        .unwrap()
}
