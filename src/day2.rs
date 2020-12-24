use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub fn solve() {
    let input: Vec<Password> = include_str!("../input/day2")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 1: {}", part2(&input));
}

fn part1(passwords: &[Password]) -> u32 {
    let mut valid_count = 0;

    for p in passwords {
        let occurrences = p.password.matches(p.letter).count();

        if occurrences >= p.min && occurrences <= p.max {
            valid_count += 1;
        }
    }

    valid_count
}

fn part2(passwords: &[Password]) -> u32 {
    let mut valid_count = 0;

    for p in passwords {
        let first = p.password.chars().nth(p.min - 1);
        let second = p.password.chars().nth(p.max - 1);

        let first_match = first.map(|c| c == p.letter).unwrap_or(false);
        let second_match = second.map(|c| c == p.letter).unwrap_or(false);

        if first_match ^ second_match {
            valid_count += 1;
        }
    }

    valid_count
}

struct Password {
    // Password policy
    min: usize,
    max: usize,
    letter: char,

    // The password
    password: String,
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Password, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let min = caps.get(1).unwrap().as_str().parse()?;
        let max = caps.get(2).unwrap().as_str().parse()?;
        let letter = caps.get(3).unwrap().as_str().parse()?;
        let password = caps.get(4).unwrap().as_str().to_string();

        Ok(Password {
            min,
            max,
            letter,
            password,
        })
    }
}
