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
type Output2 = u64;

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
        let rem = input.ready_at % b.id as u64;
        let div = input.ready_at / b.id as u64;
        if rem == 0 {
            bus_to_take = b.id as u64;
            earliest_departure = input.ready_at;
            break;
        } else {
            let first = (div + 1) * b.id as u64;
            if first < earliest_departure {
                earliest_departure = first;
                bus_to_take = b.id as u64;
            }
        }
    }
    bus_to_take * (earliest_departure - input.ready_at)
}

fn solve_part_2(input: &Input) -> Output2 {
    let (first, others) = input.buses.split_first().unwrap();
    let mut period = first.id;
    let mut t = 0;
    for b in others.iter() {
        // check each repetition of the match
        // it repeats at period_a * period_b * ... period_n
        for i in 1.. {
            // next cycle (potential match) is at t repeated each period
            let next_cycle = t + i * period;
            if (next_cycle + b.list_off) % b.id == 0 {
                t = next_cycle;
                // once we got a match for the next bus
                // we know it repeats with its combined period
                period = period * b.id;
                break;
            }
        }
    }
    t
}

fn main() {
    let input_path = get_input_path();
    let raw_input = fs::read_to_string(input_path).unwrap();
    let input = parse_input(&raw_input);
    println!("{:?}", input);
    let part_1_result = solve_part_1(&input);
    println!("Part 1: {:?}", part_1_result);
    let part_2_result = solve_part_2(&input);
    println!("Part 2: {:?}", part_2_result);
}

fn find_shifts(a: u64, b: u64, b_off: u64) -> Vec<u64> {
    let mut a_mul: Vec<u64> = Vec::new();
    for i in 1..a {
        if (b * i) % a == b_off {
            a_mul.push(((b * i) - b_off) / a);
        }
    }
    a_mul
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_2_ex_0() {
        let input = "939\n\
                     7,13,x,x,59,x,31,19";
        let input = parse_input(input);
        let res = solve_part_2(&input);
        assert_eq!(1068781, res);
    }
    #[test]
    fn part_2_ex_1() {
        let input = "0\n\
        67,7,59,61";
        let input = parse_input(input);
        let res = solve_part_2(&input);
        assert_eq!(754018, res);
    }
    #[test]
    fn part_2_ex_2() {
        let input = "0\n\
        67,x,7,59,61";
        let input = parse_input(input);
        let res = solve_part_2(&input);
        assert_eq!(779210, res);
    }
    #[test]
    fn part_2_ex_3() {
        let input = "0\n\
        67,7,x,59,61";
        let input = parse_input(input);
        let res = solve_part_2(&input);
        assert_eq!(1261476, res);
    }
    #[test]
    fn part_2_ex_4() {
        let input = "0\n\
        1789,37,47,1889";
        let input = parse_input(input);
        let res = solve_part_2(&input);
        assert_eq!(1202161486, res);
    }
}
