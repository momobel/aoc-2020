use std::{collections::HashSet, env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type YesAnswers = HashSet<char>;
type GroupAnswers = Vec<YesAnswers>;
type Input = Vec<GroupAnswers>;
type Output1 = usize;
type Output2 = usize;

fn parse_input(input: &str) -> Input {
    input
        .split("\n\n") // split on empty lines, 2 successive new lines
        .map(|group| {
            group
                .lines()
                .map(|l| {
                    l.chars().fold(YesAnswers::new(), |mut set, c| {
                        set.insert(c);
                        set
                    })
                })
                .collect::<GroupAnswers>()
        })
        .collect::<Input>()
}

fn solve_part_1(input: &Input) -> Output1 {
    input
        .iter()
        .map(|group| {
            let set = group.iter().fold(YesAnswers::new(), |mut set, answers| {
                answers.iter().for_each(|a| {
                    set.insert(*a);
                });
                set
            });
            set.len()
        })
        .sum()
}

fn solve_part_2(input: &Input) -> Output2 {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(group.iter().next().unwrap().clone(), |mut set, answers| {
                    set.retain(|a| answers.contains(a));
                    set
                })
                .len()
        })
        .sum()
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
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
    #[test]
    fn test_part1_example() {
        let input = parse_input(EXAMPLE_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(11, res);
    }
    #[test]
    fn test_part2_example() {
        let input = parse_input(EXAMPLE_INPUT);
        let res = solve_part_2(&input);
        assert_eq!(6, res);
    }
}
