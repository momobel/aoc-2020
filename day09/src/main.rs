use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Number = u64;
type Input = Vec<Number>;
type Output1 = Number;
type Output2 = Number;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.parse::<Number>().unwrap())
        .collect()
}

fn xmas_find_weakness_target(input: &Input, preamble_len: usize) -> Output1 {
    let mut cache: VecDeque<HashSet<Number>> = VecDeque::with_capacity(preamble_len);
    for a in input.iter().take(preamble_len) {
        let mut sums: HashSet<Number> = HashSet::with_capacity(preamble_len - 1);
        for b in input.iter().take(preamble_len) {
            if a != b {
                sums.insert(a + b);
            }
        }
        cache.push_back(sums);
    }
    for window in input.windows(preamble_len + 1) {
        let (to_check, leading) = window.split_last().unwrap();
        if cache.iter().find(|sums| sums.contains(to_check)).is_none() {
            return *to_check;
        }
        // prepare next iteration by:
        //  - removing sums of first value (out of scope at next iteration)
        //  - adding sums of incoming check value (in scope at next iteration)
        cache.pop_front();
        let (_, kept) = leading.split_first().unwrap();
        let mut next_sums: HashSet<Number> = HashSet::with_capacity(preamble_len - 1);
        for n in kept {
            if n != to_check {
                next_sums.insert(n + to_check);
            }
        }
        cache.push_back(next_sums);
    }
    panic!("Failed to find weakness target");
}

fn solve_part_1(input: &Input) -> Output1 {
    xmas_find_weakness_target(input, 25)
}

fn xmas_find_weakness(input: &Input, target: Number) -> Number {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut sum = input[0];
    while sum != target {
        if sum < target {
            end += 1;
            sum += input[end];
        } else {
            sum -= input[start];
            start += 1;
        }
    }
    let target_sum = &input[start..=end];
    return target_sum.iter().min().unwrap() + target_sum.iter().max().unwrap();
}

fn solve_part_2(input: &Input) -> Output2 {
    let target = xmas_find_weakness_target(input, 25);
    xmas_find_weakness(input, target)
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
        let res = xmas_find_weakness_target(&input, 5);
        assert_eq!(127, res);
    }

    #[test]
    fn text_part2_example() {
        let input = parse_input(EX_INPUT);
        let res = xmas_find_weakness(&input, 127);
        assert_eq!(62, res);
    }
}
