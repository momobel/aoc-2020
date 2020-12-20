use std::{env, error::Error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Orientation {
    North,
    South,
    East,
    West,
}
#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
enum Action {
    Move(Orientation),
    Turn(Side),
    Forward,
}
#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub action: Action,
    pub value: u32,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let action = match action {
            "N" => Action::Move(Orientation::North),
            "S" => Action::Move(Orientation::South),
            "E" => Action::Move(Orientation::East),
            "W" => Action::Move(Orientation::West),
            "L" => Action::Turn(Side::Left),
            "R" => Action::Turn(Side::Right),
            "F" => Action::Forward,
            _ => return Err(Box::<dyn Error>::from("Wrong action")),
        };
        let value: u32 = value.parse()?;
        Ok(Instruction { action, value })
    }
}

type Input = Vec<Instruction>;
type Output1 = u32;
type Output2 = u32;

#[derive(Debug, Clone, Copy)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_toward_dir(&mut self, dist: u32, dir: Orientation) {
        let dist = dist as i32;
        match dir {
            Orientation::North => self.y += dist,
            Orientation::South => self.y -= dist,
            Orientation::East => self.x += dist,
            Orientation::West => self.x -= dist,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    pub orientation: Orientation,
    pub position: Position,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            orientation: Orientation::East,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn move_toward_dir(&mut self, dist: u32, dir: Orientation) {
        self.position.move_toward_dir(dist, dir);
    }

    pub fn turn(&mut self, angle: u32, side: Side) {
        const ORIENTATIONS: &[Orientation] = &[
            Orientation::North,
            Orientation::East,
            Orientation::South,
            Orientation::West,
        ];
        let shifts = (angle % 360) / 90 - 1;
        let iter = ORIENTATIONS.iter();
        let orientation = match side {
            Side::Right => {
                let mut r_iter = iter.cycle();
                r_iter.find(|&o| *o == self.orientation).unwrap();
                r_iter.nth(shifts as usize)
            }
            Side::Left => {
                let mut l_iter = iter.rev().cycle();
                l_iter.find(|&o| *o == self.orientation).unwrap();
                l_iter.nth(shifts as usize)
            }
        };
        self.orientation = orientation.copied().unwrap();
    }
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut ship = Ship::new();
    for inst in input.iter() {
        match inst.action {
            Action::Forward => ship.move_toward_dir(inst.value, ship.orientation),
            Action::Move(orient) => ship.move_toward_dir(inst.value, orient),
            Action::Turn(side) => ship.turn(inst.value, side),
        }
    }
    ship.position.x.abs() as u32 + ship.position.y.abs() as u32
}

#[derive(Debug, Clone, Copy)]
struct Waypoint {
    pub position: Position,
}

impl Waypoint {
    pub fn new() -> Self {
        Waypoint {
            position: Position { x: 10, y: 1 },
        }
    }

    pub fn move_toward_dir(&mut self, dist: u32, orientation: Orientation) {
        self.position.move_toward_dir(dist, orientation);
    }

    pub fn rotate(&mut self, angle: u32, side: Side) {
        let angle = (angle % 360) as i32;
        let angle = match side {
            Side::Left => angle,
            Side::Right => 360 - angle,
        };
        let (cos, sin) = if angle == 90 {
            (0, 1)
        } else if angle == 180 {
            (-1, 0)
        } else if angle == 270 {
            (0, -1)
        } else {
            panic!("Unknown angle");
        };
        let x = self.position.x * cos - self.position.y * sin;
        let y = self.position.x * sin + self.position.y * cos;
        self.position.x = x;
        self.position.y = y;
    }
}

fn solve_part_2(input: &Input) -> Output2 {
    let mut ship = Ship::new();
    let mut wayp = Waypoint::new();
    for inst in input.iter() {
        match inst.action {
            Action::Move(orient) => wayp.move_toward_dir(inst.value, orient),
            Action::Forward => {
                ship.position.x += inst.value as i32 * wayp.position.x;
                ship.position.y += inst.value as i32 * wayp.position.y;
            }
            Action::Turn(side) => wayp.rotate(inst.value, side),
        }
    }
    ship.position.x.abs() as u32 + ship.position.y.abs() as u32
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
    const EX_INPUT: &str = "F10
N3
F7
R90
F11";
    #[test]
    fn test_ex_part1() {
        let input = parse_input(EX_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(25, res);
    }
    #[test]
    fn test_ex_part2() {
        let input = parse_input(EX_INPUT);
        let res = solve_part_2(&input);
        assert_eq!(286, res);
    }
}
