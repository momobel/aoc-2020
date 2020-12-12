use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

enum Instruction {
    Noop,
    Acc(i32),
    Jump(i32),
}

type Program = Vec<Instruction>;
type Input = Program;
type Output1 = i32;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let tokens: Vec<&str> = l.split(' ').collect();
            let inst = tokens.get(0).unwrap();
            let arg = tokens.get(1).unwrap().parse::<i32>().unwrap();
            match *inst {
                "nop" => Instruction::Noop,
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jump(arg),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

struct Machine<'a> {
    prog: &'a Program,
    pub acc: i32,
    pub iptr: usize,
}

impl<'a> Machine<'a> {
    pub fn new(prog: &Program) -> Machine {
        Machine {
            prog,
            acc: 0,
            iptr: 0,
        }
    }

    pub fn step(&mut self) {
        let inst = self.prog.get(self.iptr).unwrap();
        match inst {
            Instruction::Noop => self.iptr += 1,
            Instruction::Jump(off) => self.iptr = (self.iptr as i32 + *off) as usize,
            Instruction::Acc(arg) => {
                self.acc += arg;
                self.iptr += 1;
            }
        }
    }
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut executed: Vec<bool> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        executed.push(false);
    }
    let mut m: Machine = Machine::new(input);
    while !executed[m.iptr] {
        executed[m.iptr] = true;
        m.step();
    }
    m.acc
}

fn solve_part_2(input: &Input) -> Output2 {
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
    const EX_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1_ex() {
        let input = parse_input(EX_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(5, res);
    }
}
