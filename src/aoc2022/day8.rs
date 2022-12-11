fn count_up(tree_map: &Vec<Vec<isize>>, row: usize, column: usize) -> usize {
    let mut counter = 0;
    for up in (0..row).rev() {
        counter += 1;
        if tree_map[row][column] <= tree_map[up][column] {
            break;
        }
    }
    counter
}

fn count_down(tree_map: &Vec<Vec<isize>>, row: usize, column: usize) -> usize {
    let mut counter = 0;
    for down in (row + 1)..tree_map[row].len() {
        counter += 1;
        if tree_map[row][column] <= tree_map[down][column] {
            break;
        }
    }
    counter
}

fn count_left(tree_map: &Vec<Vec<isize>>, row: usize, column: usize) -> usize {
    let mut counter = 0;
    for left in (0..column).rev() {
        counter += 1;
        if tree_map[row][column] <= tree_map[row][left] {
            break;
        }
    }
    counter
}

fn count_right(tree_map: &Vec<Vec<isize>>, row: usize, column: usize) -> usize {
    let mut counter = 0;
    for right in (column + 1)..tree_map.len() {
        counter += 1;
        if tree_map[row][column] <= tree_map[row][right] {
            break;
        }
    }
    counter
}

pub fn part_1(input: String) -> usize {
    let tree_map: Vec<Vec<isize>> = input
        .split('\n')
        .map(|e| {
            e.to_string()
                .chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|e| !e.is_empty())
        .collect();
    let mut visibile_tree = 0;
    for i in 0..tree_map.len() {
        for j in 0..tree_map[i].len() {
            if i == 0 || j == 0 || i == tree_map.len() - 1 || j == tree_map[0].len() - 1 {
                visibile_tree += 1;
            } else if count_up(&tree_map, i, j) == j {
                visibile_tree += 1;
            } else if count_down(&tree_map, i, j) == tree_map.len() - j {
                visibile_tree += 1;
            } else if count_left(&tree_map, i, j) == i {
                visibile_tree += 1;
            } else if count_right(&tree_map, i, j) == tree_map[i].len() - i {
                visibile_tree += 1;
            }
        }
    }
    visibile_tree
}

pub fn part_2(input: String) -> usize {
    let tree_map: Vec<Vec<isize>> = input
        .split('\n')
        .map(|e| {
            e.to_string()
                .chars()
                .map(|c| c.to_string().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|e| !e.is_empty())
        .collect();
    let mut highest = 0;
    for i in 0..tree_map.len() {
        for j in 0..tree_map[i].len() {
            if i == 0 || j == 0 || i == tree_map.len() - 1 || j == tree_map[0].len() - 1 {
                continue;
            }
            let max_tree = count_up(&tree_map, i, j)
                * count_down(&tree_map, i, j)
                * count_left(&tree_map, i, j)
                * count_right(&tree_map, i, j);
            highest = if highest < max_tree {
                max_tree
            } else {
                highest
            }
        }
    }
    highest
}
