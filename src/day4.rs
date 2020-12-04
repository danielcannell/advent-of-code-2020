use regex::Regex;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

pub fn solve() {
    let passports = parse_passports(include_str!("../input/day4"));

    println!("Part 1: {}", part1(&passports));
    println!("Part 2: {}", part2(&passports));
}

fn part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|p| passport_valid(p, false))
        .count()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| passport_valid(p, true)).count()
}

fn passport_valid(p: &Passport, check_values: bool) -> bool {
    let rules: HashMap<_, _> = [
        ("byr", Regex::new(r"\d+").unwrap()),
        ("iyr", Regex::new(r"\d+").unwrap()),
        ("eyr", Regex::new(r"\d+").unwrap()),
        ("hgt", Regex::new(r"\d+(cm|in)").unwrap()),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap()),
        ("pid", Regex::new(r"^\d{9}$").unwrap()),
    ]
    .iter()
    .cloned()
    .collect();

    if !rules.keys().all(|&k| p.contains_key(k)) {
        return false;
    }

    if check_values {
        for (&key, re) in rules.iter() {
            if !re.is_match(&p[key]) {
                return false;
            }
        }

        let byr: u32 = p["byr"].parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr: u32 = p["iyr"].parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr: u32 = p["eyr"].parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt_len = p["hgt"].len();
        let hgt: u32 = p["hgt"][..hgt_len - 2].parse().unwrap();
        let hgt_unit = &p["hgt"][hgt_len - 2..];

        match hgt_unit {
            "in" => {
                if hgt < 59 || hgt > 76 {
                    return false;
                }
            }
            "cm" => {
                if hgt < 150 || hgt > 193 {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

pub fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut p = HashMap::new();

    let re = Regex::new(r"(\w+):([^ ]+)").unwrap();

    for line in input.lines() {
        if line.trim().len() == 0 {
            passports.push(p);
            p = HashMap::new();
        } else {
            for cap in re.captures_iter(line) {
                let key = cap.get(1).unwrap().as_str().to_string();
                let value = cap.get(2).unwrap().as_str().to_string();
                p.insert(key, value);
            }
        }
    }

    if !p.is_empty() {
        passports.push(p);
    }

    passports
}
