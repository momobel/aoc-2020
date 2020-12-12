use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Debug)]
struct Constraint {
    pub bag: String,
    pub quantity: u16,
}

#[derive(Debug)]
struct Rule {
    pub container: String,
    pub constraints: Vec<Constraint>,
}

type Input = Vec<Rule>;
type Output1 = usize;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let tokens: Vec<&str> = l.split(" bags contain ").collect();
            let container = tokens.get(0).unwrap().to_string();
            let constraints = tokens.get(1).unwrap().trim_end_matches('.');
            let constraints = match constraints {
                "no other bags" => Vec::<Constraint>::new(),
                _ => constraints
                    .split(',')
                    .map(|item| item.trim_end_matches('s').trim_end_matches("bag").trim())
                    .map(|ctext| {
                        let space_pos = ctext.find(' ').unwrap();
                        let (quantity, bag) = ctext.split_at(space_pos);
                        Constraint {
                            quantity: quantity.parse().unwrap(),
                            bag: bag.trim().to_string(),
                        }
                    })
                    .collect(),
            };
            Rule {
                container,
                constraints,
            }
        })
        .collect()
}

fn solve_part_1(input: &Input) -> Output1 {
    let bag_holders = input
        .iter()
        .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, rule| {
            rule.constraints.iter().map(|c| &c.bag).for_each(|bag| {
                map.entry(bag)
                    .and_modify(|bigger| {
                        bigger.insert(&rule.container);
                    })
                    .or_insert_with(|| [rule.container.as_str()].iter().cloned().collect());
            });
            map
        });
    let mut bigger: HashSet<&str> = HashSet::new();
    let mut bags: HashSet<&str> = HashSet::new();
    bags.insert("shiny gold");
    while !bags.is_empty() {
        let mut next: HashSet<&str> = HashSet::new();
        for b in bags.iter() {
            if let Some(holders) = bag_holders.get(b) {
                holders.iter().for_each(|h| {
                    if !bigger.contains(h) {
                        next.insert(h);
                    }
                });
            }
        }
        next.iter().for_each(|b| {
            bigger.insert(b);
        });
        bags = next;
    }
    bigger.len()
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
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_parse() {
        let input = parse_input(EXAMPLE_INPUT);
        dbg!(input);
    }

    #[test]
    fn test_part1_example() {
        let input = parse_input(EXAMPLE_INPUT);
        let res = solve_part_1(&input);
        assert_eq!(4, res);
    }
}
