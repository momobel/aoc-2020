use std::{collections::HashMap, env, error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Debug, Clone, Copy)]
struct WriteMemArgs {
    pub address: u64,
    pub value: u64,
}

#[derive(Debug, Clone, Copy)]
struct Mask {
    pub and: u64,
    pub or: u64,
}

impl Mask {
    pub fn new() -> Self {
        Self {
            or: 0,
            and: u64::MAX,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    WriteMem(WriteMemArgs),
    UpdateMask(Mask),
}

impl FromStr for Mask {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask = s.split('=').nth(1).unwrap().trim();
        let mask = mask
            .chars()
            .enumerate()
            .fold(Mask::new(), |mut mask, (idx, c)| {
                let bit = 35 - idx;
                if c == '1' {
                    mask.or |= 1 << bit;
                } else if c == '0' {
                    mask.and &= !(1 << bit);
                }
                mask
            });
        Ok(mask)
    }
}

impl FromStr for WriteMemArgs {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('=').map(|p| p.trim());
        let mem = parts.next().unwrap();
        let value: u64 = parts.next().unwrap().parse()?;
        let start_addr = mem.find('[').unwrap() + 1;
        let end_addr = mem.find(']').unwrap();
        let address: u64 = (&mem[start_addr..end_addr]).parse()?;
        Ok(WriteMemArgs { address, value })
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Self::UpdateMask(s.parse::<Mask>()?))
        } else if s.starts_with("mem") {
            Ok(Self::WriteMem(s.parse::<WriteMemArgs>()?))
        } else {
            Err(Box::from("Unknown instruction"))
        }
    }
}

type Program = Vec<Instruction>;
type Input = Program;
type Output1 = u64;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask::new();
    for inst in input.iter() {
        match inst {
            Instruction::UpdateMask(m) => mask = *m,
            Instruction::WriteMem(wr_args) => {
                let masked = (wr_args.value | mask.or) & mask.and;
                memory.insert(wr_args.address, masked);
            }
        }
    }
    memory.iter().map(|(_, val)| val).sum()
}

fn solve_part_2(input: &Input) -> Output2 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_ex() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
        mem[8] = 11\n\
        mem[7] = 101\n\
        mem[8] = 0";
        let input = parse_input(input);
        println!("{:?}", input);
        let res = solve_part_1(&input);
        assert_eq!(165, res);
    }
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
