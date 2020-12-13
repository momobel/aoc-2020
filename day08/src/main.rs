use std::{collections::HashSet, env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Clone, Copy)]
enum Operation {
    Noop,
    Acc,
    Jump,
}

#[derive(Clone, Copy)]
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
    pub acc: i32,
    pub iptr: usize,
}

#[derive(PartialEq)]
enum MachineState {
    Running,
    Terminated,
}

impl<'a> Machine<'a> {
    pub fn new(prog: &Program) -> Machine {
        Machine {
            prog,
            acc: 0,
            iptr: 0,
        }
    }

    pub fn step(&mut self) -> MachineState {
        let inst = match self.prog.get(self.iptr) {
            Some(i) => i,
            None => return MachineState::Terminated,
        };
        match inst.op {
            Operation::Noop => self.iptr += 1,
            Operation::Jump => self.iptr = (self.iptr as i32 + inst.arg) as usize,
            Operation::Acc => {
                self.acc += inst.arg;
                self.iptr += 1;
            }
        }
        MachineState::Running
    }

    pub fn state(&self) -> MachineState {
        match self.prog.get(self.iptr) {
            Some(_) => MachineState::Running,
            None => MachineState::Terminated,
        }
    }

    pub fn next_op(&self) -> Operation {
        self.prog.get(self.iptr).unwrap().op
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

fn run_best_effort(m: &mut Machine, exec_cache: &mut [bool]) -> HashSet<usize> {
    exec_cache.iter_mut().for_each(|x| *x = false);
    let mut flow_ctrl_visited: HashSet<usize> = HashSet::new();
    while m.state() == MachineState::Running && !exec_cache[m.iptr] {
        exec_cache[m.iptr] = true;
        match m.next_op() {
            Operation::Jump | Operation::Noop => {
                flow_ctrl_visited.insert(m.iptr);
            }
            _ => {}
        }
        m.step();
    }
    flow_ctrl_visited
}

fn solve_part_2(input: &Input) -> Output2 {
    let mut executed: Vec<bool> = Vec::with_capacity(input.len());
    for _ in 0..input.len() {
        executed.push(false);
    }
    let flow_ctrls = run_best_effort(&mut Machine::new(input), &mut executed);
    for to_mod in flow_ctrls {
        let mut modded = input.clone();
        let patch = modded.get_mut(to_mod).unwrap();
        match patch {
            Instruction {
                op: Operation::Noop,
                ..
            } => patch.op = Operation::Jump,
            Instruction {
                op: Operation::Jump,
                ..
            } => patch.op = Operation::Noop,
            _ => panic!("Acc instruction shall not be patched"),
        };
        let mut m: Machine = Machine::new(&modded);
        let flow_ctrl_execed = run_best_effort(&mut m, &mut executed);
        match m.state() {
            MachineState::Terminated => return m.acc,
            MachineState::Running => continue,
        }
    }
    panic!("Didn't find a proper instruction to patch");
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
