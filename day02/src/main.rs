use std::{env, error::Error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

struct PasswordPolicy {
    pub letter: char,
    pub min: u16,
    pub max: u16,
}

struct PasswordDetails {
    pub policy: PasswordPolicy,
    pub password: String,
}

impl PasswordDetails {
    fn valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count() as u16;
        count >= self.policy.min && count <= self.policy.max
    }
}

impl FromStr for PasswordPolicy {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse sub parts
        let parts: Vec<&str> = s.split(' ').collect();
        let range = parts.get(0).ok_or("Missing range specifier")?;
        let require = parts.get(1).ok_or("Missing required letter")?;
        // parse range specifier
        let range_tokens: Vec<&str> = range.split('-').collect();
        let min = range_tokens
            .get(0)
            .ok_or("Missing min specifier")?
            .parse::<u16>()?;
        let max = range_tokens
            .get(1)
            .ok_or("Missing min specifier")?
            .parse::<u16>()?;
        // parse letter requirement
        let letter = require
            .chars()
            .next()
            .ok_or("Expected a letter in the policy")?;
        // all good!
        Ok(PasswordPolicy { letter, min, max })
    }
}

impl FromStr for PasswordDetails {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(':').collect();
        let policy = tokens
            .get(0)
            .ok_or("Missing password policy")?
            .parse::<PasswordPolicy>()?;
        let password = tokens.get(1).ok_or("Missing password")?.trim().to_string();
        Ok(PasswordDetails { policy, password })
    }
}

type Input = Vec<PasswordDetails>;
type Output1 = usize;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<PasswordDetails>().unwrap())
        .collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    input.iter().filter(|p| p.valid()).count()
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
