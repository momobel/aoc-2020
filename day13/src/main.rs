use std::{env, fs};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

#[derive(Debug, Clone, Copy)]
struct Bus {
    pub id: u64,
    pub list_off: u64,
}

#[derive(Debug, Clone)]
struct RouteInfo {
    pub ready_at: u64,
    pub buses: Vec<Bus>,
}

type Input = RouteInfo;
type Output1 = u64;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let ready_at: u64 = lines.get(0).unwrap().parse().unwrap();
    let buses = lines
        .get(1)
        .copied()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, tok)| *tok != "x")
        .map(|(idx, tok)| Bus {
            id: tok.parse::<u64>().unwrap(),
            list_off: idx as u64,
        })
        .collect();
    RouteInfo { ready_at, buses }
}

fn solve_part_1(input: &Input) -> Output1 {
    let mut bus_to_take: u64 = 0;
    let mut earliest_departure: u64 = u64::MAX;
    for b in input.buses.iter() {
        let rem = input.ready_at % b.id;
        let div = input.ready_at / b.id;
        if rem == 0 {
            bus_to_take = b.id;
            earliest_departure = input.ready_at;
            break;
        } else {
            let first = (div + 1) * b.id;
            if first < earliest_departure {
                earliest_departure = first;
                bus_to_take = b.id;
            }
        }
    }
    bus_to_take * (earliest_departure - input.ready_at)
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
