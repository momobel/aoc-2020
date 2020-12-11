use std::{collections::HashMap, env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Passport = HashMap<String, String>;
type Input = Vec<Passport>;
type Output1 = usize;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    let mut passports: Vec<Passport> = Vec::new();
    let mut p: Passport = Passport::new();
    for l in input.lines() {
        if l.is_empty() {
            if p.is_empty() {
                panic!("Passport must not be empty");
            }
            passports.push(p);
            p = Passport::new();
        } else {
            l.split(' ').for_each(|kv| {
                let parts: Vec<&str> = kv.split(':').collect();
                p.insert(
                    parts.get(0).unwrap().to_string(),
                    parts.get(1).unwrap().to_string(),
                );
            });
        }
    }
    if !p.is_empty() {
        passports.push(p);
    }
    passports
}

fn solve_part_1(input: &Input) -> Output1 {
    let mandatory = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|passport| mandatory.iter().all(|key| passport.contains_key(*key)))
        .count()
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
