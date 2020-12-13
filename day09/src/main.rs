use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Number = u64;
type Input = Vec<Number>;
type Output1 = Number;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<Number>().unwrap())
        .collect()
}

fn xmas_check(leading: &[Number], next: Number) -> bool {
    let mut first = leading.iter();
    while let Some(a) = first.next() {
        for b in first.clone() {
            if a != b && a + b == next {
                return true;
            }
        }
    }
    false
}

fn xmas_find_weakness(input: &Input, preamble_len: usize) -> Output1 {
    *input
        .windows(preamble_len + 1)
        .map(|w| w.split_last().unwrap())
        .find(|(&last, heading)| !xmas_check(heading, last))
        .unwrap()
        .0
}

fn solve_part_1(input: &Input) -> Output1 {
    xmas_find_weakness(input, 25)
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

    const EX_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn text_part1_example() {
        let input = parse_input(EX_INPUT);
        let res = xmas_find_weakness(&input, 5);
        assert_eq!(127, res);
    }
}
