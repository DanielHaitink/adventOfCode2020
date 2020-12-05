use std::collections::HashMap;
use crate::input::read_input_until_empty;
use regex::Regex;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum PassportField {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

struct PassportMap {
    map: HashMap<PassportField, String>,
}

enum HeightSystem {
    Metric,
    Imperial,
    None
}

struct Height {
    height: usize,
    system: HeightSystem,
}

struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<usize>,
}

impl Passport {
    fn create_empty() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        }
    }

    fn is_valid_one(&self) -> bool {
        return !(self.height.is_none() ||
            self.passport_id.is_none() ||
            self.hair_color.is_none() ||
            self.expiration_year.is_none() ||
            self.issue_year.is_none() ||
            self.birth_year.is_none() ||
            self.eye_color.is_none())
    }

    fn is_valid_two(&self) -> bool {
        return self.is_valid_one() &&
            self.is_birth_year_valid() &&
            self.is_expiration_year_valid() &&
            self.is_eye_color_valid() &&
            self.is_hair_color_valid() &&
            self.is_height_valid() &&
            self.is_issue_year_valid() &&
            self.is_passport_id_valid()
    }

    fn is_birth_year_valid(&self) -> bool {
        return self.birth_year.is_some() && !(self.birth_year.unwrap() < 1920 || self.birth_year.unwrap() > 2002);
    }

    fn is_issue_year_valid(&self) -> bool {
        return self.issue_year.is_some() && !(self.issue_year.unwrap() < 2010 || self.issue_year.unwrap() > 2020);
    }

    fn is_expiration_year_valid(&self) -> bool {
        return self.expiration_year.is_some() && !(self.expiration_year.unwrap() < 2020 || self.expiration_year.unwrap() > 2030);
    }

    fn is_height_valid(&self) -> bool {
        if !self.height.is_some() {
            return false;
        }

        return match self.height.as_ref().unwrap().system {
            HeightSystem::Metric => !(self.height.as_ref().unwrap().height < 150 || self.height.as_ref().unwrap().height > 193),
            HeightSystem::Imperial => !(self.height.as_ref().unwrap().height < 59 || self.height.as_ref().unwrap().height > 76),
            HeightSystem::None => false
        };
    }

    fn is_hair_color_valid(&self) -> bool {
        // # followed by exactly six characters 0-9 or a-f.
        if !self.hair_color.is_some() {
            return false;
        }

        let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        return re.is_match(self.hair_color.as_ref().unwrap());
    }

    fn is_eye_color_valid(&self) -> bool {
        let valid_colors: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        return self.eye_color.is_some() && valid_colors.contains(&&**self.eye_color.as_ref().unwrap());
    }

    fn is_passport_id_valid(&self) -> bool {
        return self.passport_id.as_ref().is_some() && self.passport_id.as_ref().unwrap().len() == 9;
    }
}

pub fn day_4(part: i8) {
    let parsed_passports = parse_input();

    if part == 1 {
        part_one(&parsed_passports.0);
    } else {
        part_two(&parsed_passports.1);
    }
}

fn parse_input() -> (Vec<PassportMap>, Vec<Passport>) {
    let mut input = read_input_until_empty();
    let mut passports_map: Vec<PassportMap> = Vec::new();
    let mut passports: Vec<Passport> = Vec::new();

    while !input.is_empty() {
        let fields = get_passport_fields(&input);
        passports_map.push(parse_passport_map(&fields));
        passports.push(parse_passport(&fields));

        input = read_input_until_empty();
    }

    (passports_map, passports)
}

fn parse_passport(fields: &Vec<&str>) -> Passport {
    let mut passport = Passport::create_empty();

    for field in fields {
        let split: Vec<&str> = field.split(":").collect();
        let key = split[0];
        let value = split[1];

        match key {
            "byr" => {passport.birth_year = Some(value.parse().unwrap());},
            "iyr" => {passport.issue_year = Some(value.parse().unwrap());},
            "eyr" => {passport.expiration_year = Some(value.parse().unwrap());},
            "hgt" => {passport.height = Some(parse_height(value));},
            "hcl" => {passport.hair_color = Some(value.to_string());},
            "ecl" => {passport.eye_color = Some(value.to_string());},
            "pid" => {passport.passport_id = Some(value.to_string());},
            "cid" => {passport.country_id = Some(value.parse().unwrap());},
            &_ => {},
        }
    }

    return passport;
}

fn parse_height(string: &str) -> Height {
    let mut system = HeightSystem::None;

    if string.contains("cm") {
        system = HeightSystem::Metric;
    } else if string.contains("in") {
        system = HeightSystem::Imperial;
    } else {
        return Height {
            height: string.parse().unwrap(),
            system
        }
    }

    let height = string.split_at(string.len() - 2).0.parse().unwrap();

    return Height{
        height,
        system,
    }
}

fn parse_passport_map(fields: &Vec<&str>) -> PassportMap {
    let mut map: HashMap<PassportField, String> = HashMap::new();

    for field in fields {
        let split: Vec<&str> = field.split(":").collect();
        let key = split[0];
        let value = split[1];

        match key {
            "byr" => {map.insert(PassportField::Byr, value.to_string());},
            "iyr" => {map.insert(PassportField::Iyr, value.to_string());},
            "eyr" => {map.insert(PassportField::Eyr, value.to_string());},
            "hgt" => {map.insert(PassportField::Hgt, value.to_string());},
            "hcl" => {map.insert(PassportField::Hcl, value.to_string());},
            "ecl" => {map.insert(PassportField::Ecl, value.to_string());},
            "pid" => {map.insert(PassportField::Pid, value.to_string());},
            "cid" => {map.insert(PassportField::Cid, value.to_string());},
            &_ => {},
        }
    }

    PassportMap {map}
}

fn get_passport_fields(input: &Vec<String>) -> Vec<&str> {
    let mut fields: Vec<&str> = Vec::new();
    for line in input {
        let split: Vec<&str> = line.split(" ").collect();
        fields.extend(split.iter());
    }

    return fields;
}

fn part_one(passports: &Vec<PassportMap>) {
    println!("{}", passports.len());

    let mut valid: usize = 0;
    for passport in passports {
        let mut min_len: usize = 8;
        if !contains_optional(passport) {
            min_len -= 1;
        }

        if passport.map.len() < min_len {
            continue;
        }
        valid += 1;
    }

    println!("Valid:{}", valid);
}

fn contains_optional(passport: &PassportMap) -> bool {
    passport.map.contains_key(&PassportField::Cid)
}

fn part_two(passports: &Vec<Passport>) {
    let mut valid: usize = 0;
    for passport in passports {
        println!("{}{}{}{}{}{}{}", passport.is_passport_id_valid(), passport.is_issue_year_valid(), passport.is_height_valid(), passport.is_hair_color_valid(), passport.is_eye_color_valid(), passport.is_expiration_year_valid(), passport.is_birth_year_valid());
        if passport.is_valid_two() {
            valid += 1;
        }
    }

    println!("Valid:{}", valid);
}

