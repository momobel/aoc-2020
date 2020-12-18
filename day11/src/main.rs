use std::{
    convert::From,
    env,
    fmt::{self, Display, Write},
    fs,
    iter::Iterator,
    ops::RangeInclusive,
};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Occupation {
    Occupied,
    Empty,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Floor,
    Seat(Occupation),
}
impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'L' => Cell::Seat(Occupation::Empty),
            '#' => Cell::Seat(Occupation::Occupied),
            '.' => Cell::Floor,
            _ => panic!("Unknown cell"),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}
impl Grid {
    pub fn new() -> Grid {
        Self {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    pub fn add_cells(&mut self, cells: &[Cell]) {
        self.width = cells.len();
        self.height += 1;
        self.cells.extend_from_slice(cells);
    }

    pub fn cell_idx(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }

    pub fn cell_idx_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y * self.width + x)
    }

    pub fn neighbours<'s>(&'s self, x: usize, y: usize) -> Neighbours<'s> {
        let x_min = if x == 0 { 0 } else { -1 };
        let x_max = if x == self.width - 1 { 0 } else { 1 };
        let y_min = if y == 0 { 0 } else { -1 };
        let y_max = if y == self.height - 1 { 0 } else { 1 };
        Neighbours {
            x: x as u32,
            y: y as u32,
            dx: (x_min..=x_max),
            dy: (y_min..=y_max),
            off_x: x_min,
            off_y: y_min,
            grid: self,
        }
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("w{} h{}\n", self.width, self.height))?;
        for (idx, cell) in self.cells.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                f.write_char('\n')?;
            }
            f.write_char(match cell {
                Cell::Floor => '.',
                Cell::Seat(Occupation::Empty) => 'L',
                Cell::Seat(Occupation::Occupied) => '#',
            })?;
        }
        Ok(())
    }
}
struct Neighbours<'n> {
    x: u32,
    y: u32,
    dx: RangeInclusive<i8>,
    dy: RangeInclusive<i8>,
    off_x: i8,
    off_y: i8,
    grid: &'n Grid,
}
impl<'n> Neighbours<'n> {
    pub fn shift(&mut self) {
        self.off_x += 1;
        if self.off_x > *self.dx.end() {
            self.off_y += 1;
            self.off_x = *self.dx.start();
        }
    }
}
impl<'n> Iterator for Neighbours<'n> {
    type Item = &'n Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.off_y > *self.dy.end() {
            return None;
        }
        if self.off_x == 0 && self.off_y == 0 {
            self.shift();
        }
        let cell = self.grid.cell(
            (self.x as i32 + self.off_x as i32) as usize,
            (self.y as i32 + self.off_y as i32) as usize,
        );
        if self.off_x <= *self.dx.end() {
            self.shift();
        }
        cell
    }
}
type Input = Grid;
type Output1 = usize;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    let mut grid = Grid::new();
    for l in input.lines() {
        let cells: Vec<Cell> = l.chars().map(|c| Cell::from(c)).collect();
        grid.add_cells(&cells);
    }
    grid
}

fn simul_round(grid: &mut Grid, mirror: &mut Grid) -> bool {
    let mut change = false;
    for (idx, cell) in grid.cells.iter().enumerate() {
        if let Cell::Floor = cell {
            continue;
        }
        let neighbours = grid
            .neighbours(idx % grid.width, idx / grid.width)
            .filter(|&c| *c == Cell::Seat(Occupation::Occupied))
            .count();
        let cell = match cell {
            Cell::Seat(Occupation::Empty) => {
                if neighbours == 0 {
                    change = true;
                    Cell::Seat(Occupation::Occupied)
                } else {
                    *cell
                }
            }
            Cell::Seat(Occupation::Occupied) => {
                if neighbours >= 4 {
                    change = true;
                    Cell::Seat(Occupation::Empty)
                } else {
                    *cell
                }
            }
            _ => *cell,
        };
        *mirror.cell_idx_mut(idx).unwrap() = cell;
    }
    grid.cells.swap_with_slice(mirror.cells.as_mut_slice());
    change
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut grid = input.clone();
    let mut mirror = input.clone();
    let mut change = true;
    while change {
        change = simul_round(&mut grid, &mut mirror);
    }
    grid.cells
        .iter()
        .filter(|&c| *c == Cell::Seat(Occupation::Occupied))
        .count()
}

fn solve_part_2(_input: &Input) -> Output2 {
    unimplemented!()
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

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    #[test]
    fn test_parse_input() {
        let input = parse_input(EXAMPLE);
        println!("{}", input);
    }
    #[test]
    fn test_simul() {
        let mut grid = parse_input(EXAMPLE);
        let mut mirror = grid.clone();
        println!("0. {}\n", grid);
        simul_round(&mut grid, &mut mirror);
        println!("1. {}\n", grid);
        simul_round(&mut grid, &mut mirror);
        println!("2. {}\n", grid);
    }
    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE);
        let res = solve_part_1(&input);
        assert_eq!(37, res);
    }
}
