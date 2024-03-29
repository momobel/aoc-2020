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
type Output2 = usize;

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

const SHINY: &str = "shiny gold";

fn solve_part_1(input: &Input) -> Output1 {
    // build map of bags to list of bags that can contain it
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
    let mut bags_visitor: HashSet<&str> = HashSet::new();
    bags_visitor.insert(SHINY);
    // go through each visited bags until there isn't any bigger
    while !bags_visitor.is_empty() {
        let mut next: HashSet<&str> = HashSet::new();
        // go through their possible container (bigger bags)
        for b in bags_visitor.iter() {
            if let Some(holders) = bag_holders.get(b) {
                // mark them as to visit next
                // add them to the bigger bags list
                holders.iter().for_each(|h| {
                    if !bigger.contains(h) {
                        next.insert(h);
                        bigger.insert(h);
                    }
                });
            }
        }
        // now visit the discovered bigger bags
        bags_visitor = next;
    }
    bigger.len()
}

fn solve_part_2(input: &Input) -> Output2 {
    let bag_constraints: HashMap<&str, &[Constraint]> = input
        .iter()
        .map(|rule| (rule.container.as_str(), rule.constraints.as_slice()))
        .collect();
    let mut contained: usize = 0;
    let mut bags_visitor: HashMap<&str, usize> = HashMap::new();
    bags_visitor.insert(SHINY, 1);
    // go through each bags to look into until there is no content
    while !bags_visitor.is_empty() {
        let mut next: HashMap<&str, usize> = HashMap::new();
        // get each visited constraints
        for (visiting, qty) in bags_visitor.iter() {
            if let Some(constraints) = bag_constraints.get(visiting) {
                // count additional bags
                // also mark the new content as to be visited next
                for c in constraints.iter() {
                    let inside = qty * c.quantity as usize;
                    contained += inside;
                    next.entry(&c.bag)
                        .and_modify(|count| *count += inside)
                        .or_insert(inside);
                }
            }
        }
        bags_visitor = next;
    }
    contained
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

    #[test]
    fn test_part2_example() {
        let input = parse_input(EXAMPLE_INPUT);
        let res = solve_part_2(&input);
        assert_eq!(32, res);
    }
}
