use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type Jolt = u32;
type Input = Vec<Jolt>;
type Output1 = usize;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input.lines().map(|l| l.parse::<Jolt>().unwrap()).collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut sorted = vec![0];
    sorted.extend(input);
    sorted.sort_by(|a, b| a.cmp(b));
    let sumed =
        sorted
            .iter()
            .zip(sorted.iter().skip(1))
            .fold((0, 0), |(ones, threes), (prev, next)| {
                let diff = next - prev;
                if diff == 1 {
                    (ones + 1, threes)
                } else if diff == 3 {
                    (ones, threes + 1)
                } else {
                    (ones, threes)
                }
            });
    sumed.0 * (sumed.1 + 1)
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
    const EX1_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";
    const EX2_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_ex1_part1() {
        let input = parse_input(EX1_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(7 * 5, res);
    }

    #[test]
    fn test_ex2_part1() {
        let input = parse_input(EX1_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(22 * 10, res);
    }
}
