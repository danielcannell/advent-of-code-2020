use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../input/day16");

    let notes = parse_input(input);

    println!("Part 1: {}", part1(&notes));
    println!("Part 2: {}", part2(&notes));
}

fn part1(notes: &Notes) -> u32 {
    let mut rate = 0;

    for t in &notes.nearby_tickets {
        rate += scanning_error_rate(t, &notes.fields);
    }

    rate
}

fn part2(notes: &Notes) -> u64 {
    let tickets: Vec<Vec<u32>> = notes
        .nearby_tickets
        .iter()
        .filter(|&ticket| {
            for &v in ticket {
                let mut any_valid = false;

                for f in &notes.fields {
                    if f.is_valid(v) {
                        any_valid = true;
                        break;
                    }
                }

                if !any_valid {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    let mut columns: Vec<Vec<u32>> = vec![Vec::new(); notes.fields.len()];
    for t in &tickets {
        for (i, &v) in t.iter().enumerate() {
            columns[i].push(v);
        }
    }

    let mut assignments = vec![usize::MAX; notes.fields.len()];
    let mut remaining = notes.fields.len();

    while remaining > 0 {
        for (col_idx, c) in columns.iter().enumerate() {
            let mut num_valid_fields = 0;
            let mut valid_field_idx = usize::MAX;

            for (field_idx, f) in notes.fields.iter().enumerate() {
                // Check if this field has already been assigned
                if assignments[field_idx] != usize::MAX {
                    continue;
                }

                // Check if this field is valid for this column
                if !f.all_valid(c) {
                    continue;
                }

                num_valid_fields += 1;
                valid_field_idx = field_idx;
            }

            if num_valid_fields == 1 {
                assignments[valid_field_idx] = col_idx;
                remaining -= 1;
            }
        }
    }

    let mut result = 1;

    for (field_idx, field) in notes.fields.iter().enumerate() {
        if field.name.starts_with("departure") {
            result *= notes.your_ticket[assignments[field_idx]] as u64;
        }
    }

    result
}

fn scanning_error_rate(ticket: &[u32], fields: &[Field]) -> u32 {
    let mut rate = 0;

    for v in ticket {
        let mut any_valid = false;

        for f in fields {
            if f.is_valid(*v) {
                any_valid = true;
                break;
            }
        }

        if !any_valid {
            rate += v;
        }
    }

    rate
}

fn parse_input(input: &str) -> Notes {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let fields: Vec<Field> = sections[0].lines().map(|l| l.parse().unwrap()).collect();

    let your_ticket: Vec<u32> = sections[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<u32>> = sections[2]
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    Notes {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

struct Notes {
    fields: Vec<Field>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

#[derive(Debug)]
struct Field {
    name: String,
    spans: Vec<(u32, u32)>,
}

impl Field {
    fn is_valid(&self, value: u32) -> bool {
        for s in &self.spans {
            if value >= s.0 && value <= s.1 {
                return true;
            }
        }

        false
    }

    fn all_valid(&self, values: &[u32]) -> bool {
        for &v in values {
            if !self.is_valid(v) {
                return false;
            }
        }

        true
    }
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Field, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let min1 = caps.get(2).unwrap().as_str().parse()?;
        let max1 = caps.get(3).unwrap().as_str().parse()?;
        let min2 = caps.get(4).unwrap().as_str().parse()?;
        let max2 = caps.get(5).unwrap().as_str().parse()?;

        Ok(Field {
            name,
            spans: vec![(min1, max1), (min2, max2)],
        })
    }
}
