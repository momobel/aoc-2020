use std::{collections::HashMap, env, error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

const WORD_LEN: usize = 36;

#[derive(Debug, Clone, Copy)]
struct WriteMemArgs {
    pub address: u64,
    pub value: u64,
}

#[derive(Debug, Clone, Copy)]
struct Mask {
    pub ones: u64,
    pub zeroes: u64,
    pub xes: u64,
}

impl Mask {
    pub fn new() -> Self {
        Self {
            ones: 0,
            zeroes: 0,
            xes: 0,
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
                let bit = WORD_LEN - 1 - idx;
                if c == '1' {
                    mask.ones |= 1 << bit;
                } else if c == '0' {
                    mask.zeroes |= 1 << bit;
                } else if c == 'X' {
                    mask.xes |= 1 << bit;
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
type Output2 = u64;

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
                let masked = (wr_args.value | mask.ones) & !mask.zeroes;
                memory.insert(wr_args.address, masked);
            }
        }
    }
    memory.iter().map(|(_, val)| val).sum()
}

fn solve_part_2(input: &Input) -> Output2 {
    let mut mask = Mask {
        zeroes: u64::MAX,
        ones: 0,
        xes: 0,
    };
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for inst in input.iter() {
        match inst {
            Instruction::UpdateMask(m) => mask = *m,
            Instruction::WriteMem(args) => {
                let changed_addr = args.address | mask.ones;
                for addr in FloatingAddrIter::new(changed_addr, mask.xes) {
                    memory.insert(addr, args.value);
                }
            }
        }
    }
    memory.iter().map(|(_, val)| val).sum()
}

#[derive(Debug, Clone, Copy)]
struct FloatingAddrIter {
    addr: u64,
    floating: u64,
    set_float_bits: u8,
    counter: u64,
}

impl FloatingAddrIter {
    pub fn new(addr: u64, floating: u64) -> FloatingAddrIter {
        let set_float_bits: u8 = (0..WORD_LEN)
            .map(|i| if (floating & (1 << i)) != 0 { 1 } else { 0 })
            .sum();
        FloatingAddrIter {
            addr,
            floating,
            set_float_bits,
            counter: 0,
        }
    }
}

impl Iterator for FloatingAddrIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let max_counter = 1 << self.set_float_bits;
        if self.counter >= max_counter {
            return None;
        }
        let mut addr = self.addr;

        for (idx, bit) in (0..WORD_LEN)
            .filter(|b| self.floating & (1 << b) != 0)
            .enumerate()
        {
            let counter_bit = self.counter & (1 << idx);
            if counter_bit == 0 {
                addr &= !(1 << bit); // set bit to 0
            } else {
                addr |= 1 << bit; // set bit to 1
            }
        }
        self.counter += 1;
        Some(addr)
    }
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
    #[test]
    fn part_2_iter() {
        let mut iter = FloatingAddrIter::new(0x1a, 0x21);
        assert_eq!(Some(26), iter.next());
        assert_eq!(Some(27), iter.next());
        assert_eq!(Some(58), iter.next());
        assert_eq!(Some(59), iter.next());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn part_2_ex() {
        let input = "mask = 000000000000000000000000000000X1001X\n\
        mem[42] = 100\n\
        mask = 00000000000000000000000000000000X0XX\n\
        mem[26] = 1";
        let input = parse_input(input);
        println!("{:?}", input);
        let res = solve_part_2(&input);
        assert_eq!(208, res);
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
