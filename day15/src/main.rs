use std::{collections::HashMap, env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Number = u32;
type Input = Vec<Number>;
type Output1 = u32;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .split(',')
        .map(|tok| tok.parse::<u32>().unwrap())
        .collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut game: HashMap<Number, usize> = HashMap::new();
    for (idx, &num) in input.iter().enumerate() {
        game.entry(num).or_insert(idx);
    }
    let mut next = 0;
    for i in game.len()..(2020 - 1) {
        match game.get(&next) {
            Some(&last_seen) => {
                game.insert(next, i);
                next = (i - last_seen) as u32;
            }
            None => {
                game.insert(next, i);
                next = 0;
            }
        }
    }
    next
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
    #[test]
    fn test_part1_ex0() {
        let starting = "0,3,6";
        let input = parse_input(starting);
        let res = solve_part_1(&input);
        assert_eq!(436, res);
    }
}
