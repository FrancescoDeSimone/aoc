use std::collections::HashMap;
use std::error::Error;

const EMPTY_CELL: char = '.';
const MARKED_CELL: char = '#';

struct Grid {
    cells: Vec<Vec<char>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn new(input: String) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let x_size = cells.len();
        let y_size = cells[0].len();

        Grid {
            cells,
            x_size,
            y_size,
        }
    }

    fn mark_cell(&mut self, x: usize, y: usize) {
        self.cells[x][y] = MARKED_CELL;
    }

    fn count_marked_cells(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|&&c| c == MARKED_CELL)
            .count()
    }

    fn count_non_empty_cells(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|&&c| c != EMPTY_CELL)
            .count()
    }
}

fn parse_input(input: String) -> (HashMap<char, Vec<(usize, usize)>>, Grid) {
    let grid = Grid::new(input.clone());
    let antennas = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, ch)| {
                if ch != EMPTY_CELL {
                    map.entry(ch)
                        .or_insert_with(Vec::new)
                        .push((row_idx, col_idx));
                }
            });

            map
        });

    (antennas, grid)
}

fn find_antinodes(antennas: &HashMap<char, Vec<(usize, usize)>>, grid: &mut Grid) {
    antennas.iter().for_each(|(sym, positions)| {
        positions.iter().for_each(|antenna1| {
            positions.iter().for_each(|antenna2| {
                if antenna1 == antenna2 {
                    return;
                }

                let (x1, y1) = (antenna1.0 as isize, antenna1.1 as isize);
                let (x2, y2) = (antenna2.0 as isize, antenna2.1 as isize);

                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut nx1 = x1 - dx;
                let mut ny1 = y1 - dy;

                let mut nx2 = x2 + dx;
                let mut ny2 = y2 + dy;

                let mut antinodes = Vec::new();

                while nx1 >= 0
                    && nx1 < grid.x_size as isize
                    && ny1 >= 0
                    && ny1 < grid.y_size as isize
                {
                    antinodes.push((nx1 as usize, ny1 as usize));
                    nx1 -= dx;
                    ny1 -= dy;
                }

                while nx2 >= 0
                    && nx2 < grid.x_size as isize
                    && ny2 >= 0
                    && ny2 < grid.y_size as isize
                {
                    antinodes.push((nx2 as usize, ny2 as usize));
                    nx2 += dx;
                    ny2 += dy;
                }

                antinodes.iter().for_each(|(nx, ny)| {
                    if grid.cells[*nx][*ny] != *sym {
                        grid.mark_cell(*nx, *ny);
                    }
                });
            });
        });
    });
}

pub fn part_1(input: String) -> usize {
    let (antennas, mut grid) = parse_input(input);
    find_antinodes(&antennas, &mut grid);
    grid.count_marked_cells()
}

pub fn part_2(input: String) -> usize {
    let (antennas, mut grid) = parse_input(input);
    find_antinodes(&antennas, &mut grid);
    grid.count_non_empty_cells()
}

#[test]
fn check_part1_1() {
    let oracle = 2;
    let input = std::fs::read_to_string("src/aoc2024/tests/day8.part1_1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part1_2() {
    let oracle = 14;
    let input = std::fs::read_to_string("src/aoc2024/tests/day8.part1").unwrap();
    assert_eq!(oracle, part_1(input));
}

#[test]
fn check_part2() {
    let oracle = 34;
    let input = std::fs::read_to_string("src/aoc2024/tests/day8.part2").unwrap();
    assert_eq!(oracle, part_2(input));
}
