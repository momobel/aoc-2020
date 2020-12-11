use std::{collections::HashMap, convert::TryFrom, env, error::Error, fs, str::FromStr};

fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).unwrap().clone()
}

type RawPassport = HashMap<String, String>;
type Input = Vec<RawPassport>;
type Output1 = usize;
type Output2 = usize;

fn parse_input(input: &str) -> Input {
    let mut passports: Vec<RawPassport> = Vec::new();
    let mut p: RawPassport = RawPassport::new();
    for l in input.lines() {
        if l.is_empty() {
            if p.is_empty() {
                panic!("Passport must not be empty");
            }
            passports.push(p);
            p = RawPassport::new();
        } else {
            l.split(' ').for_each(|kv| {
                let parts: Vec<&str> = kv.split(':').collect();
                p.insert(
                    parts.get(0).unwrap().to_string(),
                    parts.get(1).unwrap().to_string(),
                );
            });
        }
    }
    if !p.is_empty() {
        passports.push(p);
    }
    passports
}

fn solve_part_1(input: &Input) -> Output1 {
    let mandatory = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|passport| mandatory.iter().all(|key| passport.contains_key(*key)))
        .count()
}

struct Year(u16);
enum HeightUnit {
    Centimeter,
    Inch,
}
struct Height {
    length: u8,
    unit: HeightUnit,
}
struct HairColor(String);
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}
struct PassportID(String);

impl FromStr for Year {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::<dyn Error>::from("Year must be 4 digits"));
        }
        let val: u16 = s.parse()?;
        Ok(Year(val))
    }
}

impl FromStr for HeightUnit {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(HeightUnit::Centimeter),
            "in" => Ok(HeightUnit::Inch),
            _ => Err(Box::<dyn Error>::from("Height unit must be cm or in")),
        }
    }
}

impl FromStr for Height {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit_pos = s
            .find(|c: char| c.is_alphabetic())
            .ok_or("Height should contain unit")?;
        let (height, unit) = s.split_at(unit_pos);
        let unit = unit.parse()?;
        let height: u8 = height.parse()?;
        let (min, max) = match unit {
            HeightUnit::Centimeter => (150, 193),
            HeightUnit::Inch => (59, 76),
        };
        if height < min || height > max {
            return Err(Box::<dyn Error>::from("Height must be within range"));
        }
        Ok(Height {
            length: height,
            unit,
        })
    }
}

impl FromStr for HairColor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(Box::<dyn Error>::from("Hair color must be of size 7"));
        }
        let mut chars = s.chars();
        if chars.next().unwrap() != '#' {
            return Err(Box::<dyn Error>::from("Hair color must start with #"));
        }
        let color = chars.clone();
        if chars.any(|c| !c.is_ascii_hexdigit()) {
            return Err(Box::<dyn Error>::from(
                "Hair color must be 6 hexadecimal chars",
            ));
        }
        Ok(HairColor(color.collect()))
    }
}

impl FromStr for EyeColor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amb),
            "blu" => Ok(Self::Blu),
            "brn" => Ok(Self::Brn),
            "gry" => Ok(Self::Gry),
            "grn" => Ok(Self::Grn),
            "hzl" => Ok(Self::Hzl),
            "oth" => Ok(Self::Oth),
            _ => Err(Box::<dyn Error>::from("Eye color is not valid")),
        }
    }
}

impl FromStr for PassportID {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(Box::<dyn Error>::from("Passport ID must be a 9 numbers"));
        }
        if s.chars().any(|c| !c.is_ascii_digit()) {
            return Err(Box::<dyn Error>::from(
                "Passport ID must contain only numbers",
            ));
        }
        Ok(PassportID(s.to_string()))
    }
}

struct Passport {
    birth_year: Year,
    issue_year: Year,
    exp_year: Year,
    height: Height,
    hair_color: HairColor,
    eye_color: EyeColor,
    id: PassportID,
}

impl TryFrom<&RawPassport> for Passport {
    type Error = Box<dyn Error>;
    fn try_from(raw: &RawPassport) -> Result<Self, Self::Error> {
        let birth_year: Year = raw.get("byr").ok_or("Missing byr")?.parse()?;
        let issue_year: Year = raw.get("iyr").ok_or("Missing iyr")?.parse()?;
        let exp_year: Year = raw.get("eyr").ok_or("Missing eyr")?.parse()?;
        let height: Height = raw.get("hgt").ok_or("Missing hgt")?.parse()?;
        let hair_color: HairColor = raw.get("hcl").ok_or("Missing hcl")?.parse()?;
        let eye_color: EyeColor = raw.get("ecl").ok_or("Missing ecl")?.parse()?;
        let id: PassportID = raw.get("pid").ok_or("Missing pid")?.parse()?;
        if !(birth_year.0 >= 1920 && birth_year.0 <= 2002) {
            return Err(Box::<dyn Error>::from("Invalid byr"));
        }
        if !(issue_year.0 >= 2010 && issue_year.0 <= 2020) {
            return Err(Box::<dyn Error>::from("Invalid iyr"));
        }
        if !(exp_year.0 >= 2020 && exp_year.0 <= 2030) {
            return Err(Box::<dyn Error>::from("Invalid eyr"));
        }
        Ok(Passport {
            birth_year,
            issue_year,
            exp_year,
            height,
            hair_color,
            eye_color,
            id,
        })
    }
}

fn solve_part_2(input: &Input) -> Output2 {
    input
        .iter()
        .filter_map(|raw| Passport::try_from(raw).ok())
        .count()
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
