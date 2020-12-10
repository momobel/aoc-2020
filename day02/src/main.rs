use std::{env, error::Error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

struct PasswordPolicy {
    pub letter: char,
    pub indic1: u16,
    pub indic2: u16,
}

struct PasswordDetails {
    pub policy: PasswordPolicy,
    pub password: String,
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
        let indic1 = range_tokens
            .get(0)
            .ok_or("Missing indic1 specifier")?
            .parse::<u16>()?;
        let indic2 = range_tokens
            .get(1)
            .ok_or("Missing indic1 specifier")?
            .parse::<u16>()?;
        // parse letter requirement
        let letter = require
            .chars()
            .next()
            .ok_or("Expected a letter in the policy")?;
        // all good!
        Ok(PasswordPolicy {
            letter,
            indic1,
            indic2,
        })
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
type Output2 = usize;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<PasswordDetails>().unwrap())
        .collect()
}

fn pwd_valid_part_1(pwd: &PasswordDetails) -> bool {
    let count = pwd
        .password
        .chars()
        .filter(|c| *c == pwd.policy.letter)
        .count() as u16;
    count >= pwd.policy.indic1 && count <= pwd.policy.indic2
}

fn solve_part_1(input: &Input) -> Output1 {
    input.iter().filter(|p| pwd_valid_part_1(p)).count()
}

fn pwd_valid_part_2(pwd: &PasswordDetails) -> bool {
    fn has_letter_at_pos(s: &str, pos: usize, expect: char) -> bool {
        s.get(pos..=pos)
            .map(|s| s.chars().next())
            .flatten()
            .map(|c| c == expect)
            .unwrap_or(false)
    }
    let pass = &pwd.password;
    has_letter_at_pos(pass, pwd.policy.indic1 as usize - 1, pwd.policy.letter)
        ^ has_letter_at_pos(pass, pwd.policy.indic2 as usize - 1, pwd.policy.letter)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pwd_valid_part_2_ex_1() {
        let policy = PasswordPolicy {
            letter: 'a',
            indic1: 1,
            indic2: 3,
        };
        let p = PasswordDetails {
            password: String::from("abcde"),
            policy,
        };
        assert!(pwd_valid_part_2(&p));
    }
    #[test]
    fn test_pwd_valid_part_2_ex_2() {
        let policy = PasswordPolicy {
            letter: 'b',
            indic1: 1,
            indic2: 3,
        };
        let p = PasswordDetails {
            password: String::from("cdefg"),
            policy,
        };
        assert!(!pwd_valid_part_2(&p));
    }
    #[test]
    fn test_pwd_valid_part_2_ex_3() {
        let policy = PasswordPolicy {
            letter: 'c',
            indic1: 2,
            indic2: 9,
        };
        let p = PasswordDetails {
            password: String::from("ccccccccc"),
            policy,
        };
        assert!(!pwd_valid_part_2(&p));
    }
}

fn solve_part_2(input: &Input) -> Output2 {
    input.iter().filter(|p| pwd_valid_part_2(p)).count()
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
