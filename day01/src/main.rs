use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Input = Vec<u32>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

fn part_1(input: &Input) -> u32 {
    let mut first = input.iter();
    while let Some(a) = first.next() {
        let mut second = first.clone();
        while let Some(b) = second.next() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("Solution not found");
}

fn part_2(input: &Input) -> u32 {
    let mut first = input.iter();
    while let Some(a) = first.next() {
        let mut second = first.clone();
        while let Some(b) = second.next() {
            let mut third = second.clone();
            while let Some(c) = third.next() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("Solution not found");
}

fn main() {
    let input_path = get_input_path();
    let raw_input = fs::read_to_string(input_path).unwrap();
    let input = parse_input(&raw_input);
    let part_1_result = part_1(&input);
    println!("Part 1: {}", part_1_result);
    let part_2_result = part_2(&input);
    println!("Part 2: {}", part_2_result);
}
