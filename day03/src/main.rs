use std::{env, error::Error, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Clone, Copy)]
enum Cell {
    Open,
    Tree,
}

struct Grid {
    pattern: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn at(&self, x: usize, y: usize) -> Option<Cell> {
        let x = x % self.width;
        self.pattern.get(y * self.width + x).cloned()
    }
}

type Input = Grid;
type Output1 = usize;
type Output2 = usize;

fn parse_input(input: &str) -> Input {
    let mut pattern: Vec<Cell> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for l in input.lines() {
        let line_cells: Vec<Cell> = l
            .chars()
            .map(|c| match c {
                '.' => Cell::Open,
                '#' => Cell::Tree,
                _ => panic!("Not a cell: {}", c),
            })
            .collect();
        if width == 0 {
            width = line_cells.len();
        }
        pattern.extend(line_cells);
        height += 1;
    }
    Grid {
        pattern,
        width,
        height,
    }
}

fn encountered_trees(input: &Input, slope: (usize, usize)) -> usize {
    let mut trees: Output1 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    while let Some(cell) = input.at(x, y) {
        if let Cell::Tree = cell {
            trees += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    trees
}

fn solve_part_1(input: &Input) -> Output1 {
    encountered_trees(input, (3, 1))
}

fn solve_part_2(input: &Input) -> Output2 {
    let slopes: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .fold(1, |acc, slope| acc * encountered_trees(input, *slope))
}

fn main() {
    let input_path = get_input_path();
    let raw_input = fs::read_to_string(input_path).unwrap();
    let input = parse_input(&raw_input);
    let part_1_result = solve_part_1(&input);
    println!("Part 1: {:?}", part_1_result);
    let part_2_result = solve_part_2(&input);
    println!("Part 2: {:?}", part_2_result);
}
