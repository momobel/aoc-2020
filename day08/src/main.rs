use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Noop,
    Acc,
    Jump,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub op: Operation,
    pub arg: i32,
}

type Program = Vec<Instruction>;
type Input = Program;
type Output1 = i32;
type Output2 = i32;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let tokens: Vec<&str> = l.split(' ').collect();
            let inst = tokens.get(0).unwrap();
            let arg = tokens.get(1).unwrap().parse::<i32>().unwrap();
            let op = match *inst {
                "nop" => Operation::Noop,
                "acc" => Operation::Acc,
                "jmp" => Operation::Jump,
                _ => panic!("Unknown operation"),
            };
            Instruction { op, arg }
        })
        .collect()
}

struct Machine<'a> {
    prog: &'a Program,
    pub state: MachineState,
}

#[derive(Clone, Copy)]
struct MachineState {
    pub acc: i32,
    pub iptr: usize,
}

#[derive(PartialEq)]
enum ProgramState {
    Running,
    Terminated,
}

impl<'a> Machine<'a> {
    pub fn new(prog: &Program) -> Machine {
        let state = MachineState { iptr: 0, acc: 0 };
        Self::with_state(prog, state)
    }

    pub fn with_state(prog: &Program, state: MachineState) -> Machine {
        Machine { prog, state }
    }

    pub fn step(&mut self) -> ProgramState {
        let inst = match self.prog.get(self.state.iptr) {
            Some(i) => i,
            None => return ProgramState::Terminated,
        };
        match inst.op {
            Operation::Noop => self.state.iptr += 1,
            Operation::Jump => self.state.iptr = (self.state.iptr as i32 + inst.arg) as usize,
            Operation::Acc => {
                self.state.acc += inst.arg;
                self.state.iptr += 1;
            }
        }
        self.program_state()
    }

    pub fn program_state(&self) -> ProgramState {
        match self.prog.get(self.state.iptr) {
            Some(_) => ProgramState::Running,
            None => ProgramState::Terminated,
        }
    }

    pub fn next_op(&self) -> Operation {
        self.prog.get(self.state.iptr).unwrap().op
    }
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut executed: Vec<bool> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        executed.push(false);
    }
    let mut m: Machine = Machine::new(input);
    while !executed[m.state.iptr] {
        executed[m.state.iptr] = true;
        m.step();
    }
    m.state.acc
}

fn program_loops(m: &mut Machine, exec_cache: &mut [bool]) -> bool {
    while !exec_cache[m.state.iptr] {
        exec_cache[m.state.iptr] = true;
        if let ProgramState::Terminated = m.step() {
            return false;
        }
    }
    true
}

fn solve_part_2(input: &Input) -> Output2 {
    let mut executed: Vec<bool> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        executed.push(false);
    }
    let mut mach = Machine::new(&input);
    let mut modded = input.clone();
    loop {
        // step until next branching
        while mach.next_op() != Operation::Jump && mach.next_op() != Operation::Noop {
            mach.step();
        }
        // patch code before continuing
        let op: &mut Operation = &mut modded[mach.state.iptr].op;
        match op {
            Operation::Jump => *op = Operation::Noop,
            Operation::Noop => *op = Operation::Jump,
            _ => panic!("Unexpected instruction to patch"),
        }
        // create alternative machine with patched code
        let mut alternative = Machine::with_state(&modded, mach.state);
        if program_loops(&mut alternative, &mut executed) {
            executed.iter_mut().for_each(|e| *e = false);
            // step real machine on original flow control instruction
            mach.step();
        } else {
            return alternative.state.acc;
        }
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

    #[test]
    fn test_part2_ex() {
        let input = parse_input(EX_INPUT);
        let res = solve_part_2(&input);
        assert_eq!(8, res);
    }
}
