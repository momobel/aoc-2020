use std::{env, error::Error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

struct Seat {
    row: u8,
    col: u8,
}
type Input = Vec<Seat>;
type Output1 = u16;
type Output2 = u16;

#[derive(Debug)]
enum BinPart {
    Low,
    High,
}

fn parse_bin_partitions(s: &str, low: char, high: char) -> Vec<BinPart> {
    s.chars()
        .map(|c| {
            if c == low {
                BinPart::Low
            } else if c == high {
                BinPart::High
            } else {
                panic!("Wrong bin split char")
            }
        })
        .collect()
}

fn bin_space_value(splits: &[BinPart]) -> u8 {
    let mut lower = 0;
    let mut upper = 2u8.pow(splits.len() as u32) - 1;
    splits.iter().for_each(|val| {
        let diff = upper - lower;
        let mid = lower + diff / 2;
        match val {
            BinPart::Low => upper = mid,
            BinPart::High => lower = mid + diff % 2,
        }
    });
    if lower != upper {
        panic!("Partitioning didn't converge");
    }
    lower
}

impl FromStr for Seat {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fb, lr) = s.split_at(7);
        let fb = parse_bin_partitions(fb, 'F', 'B');
        let lr = parse_bin_partitions(lr, 'L', 'R');
        let row = bin_space_value(&fb);
        let col = bin_space_value(&lr);
        Ok(Seat { row, col })
    }
}

fn parse_input(input: &str) -> Input {
    input.lines().map(|l| l.parse::<Seat>().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_seat_calc() {
        let input = "FBFBBFFRLR";
        let seat: Seat = input.parse().unwrap();
        assert_eq!(44, seat.row);
        assert_eq!(5, seat.col);
    }
}

fn seat_id(seat: &Seat) -> u16 {
    seat.row as u16 * 8 + seat.col as u16
}

fn solve_part_1(input: &Input) -> Output1 {
    input.iter().map(|s| seat_id(s)).max().unwrap()
}

fn solve_part_2_old(input: &Input) -> Output2 {
    let mut seat_ids: Vec<u16> = input.iter().map(|s| seat_id(s)).collect();
    seat_ids.sort();
    let head = seat_ids.iter();
    let tail = seat_ids.iter().skip(1);
    head.zip(tail).find(|(&h, &t)| t > h + 1).unwrap().0 + 1
}

fn solve_part_2(input: &Input) -> Output2 {
    // 1 to N sum is N(N+1)/2
    // sum of all seat IDs up to the max is a sum of:
    // - all unused front seats
    // - all used seats
    // - our seat
    // simply substract front unused and used from the max sum
    let min = input.iter().map(|s| seat_id(s)).min().unwrap() as u64;
    let max = input.iter().map(|s| seat_id(s)).max().unwrap() as u64;
    let used_seats_id_sum: u64 = input.iter().map(|s| seat_id(s) as u64).sum();
    let max_id_sum = max * (max + 1) / 2;
    let min_id_sum = (min - 1) * min / 2;
    (max_id_sum - used_seats_id_sum - min_id_sum) as u16
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
