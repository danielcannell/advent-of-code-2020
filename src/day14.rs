use anyhow::{bail, Error};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve() {
    let input: Vec<Instr> = include_str!("../input/day14")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(program: &[Instr]) -> u64 {
    let mut mem = HashMap::new();
    let mut current_mask = 0;
    let mut current_value = 0;

    for instr in program {
        match instr {
            Instr::Mem { addr, value } => {
                mem.insert(*addr, (value & current_mask) | current_value);
            }

            Instr::Mask { mask, value } => {
                current_mask = *mask;
                current_value = *value;
            }
        }
    }

    mem.values().sum::<u64>()
}

fn part2(program: &[Instr]) -> u64 {
    let mut mem = HashMap::new();
    let mut current_mask = 0;
    let mut current_value = 0;

    for instr in program {
        match instr {
            Instr::Mem { addr, value } => {
                let addr = (addr | current_value) & !current_mask;

                for a in addrs_for_mask(current_mask) {
                    mem.insert(addr | a, *value);
                }
            }

            Instr::Mask { mask, value } => {
                current_mask = *mask;
                current_value = *value;
            }
        }
    }

    mem.values().sum::<u64>()
}

fn addrs_for_mask(mask: u64) -> Vec<u64> {
    if mask == 0 {
        return vec![0];
    }

    let sub_addrs = addrs_for_mask(mask & (mask - 1));

    let mut bit = 1u64;
    while mask & bit == 0 {
        bit <<= 1;
    }

    let mut addrs = sub_addrs.clone();

    for a in sub_addrs {
        addrs.push(a | bit);
    }

    addrs
}

#[derive(Debug)]
enum Instr {
    Mem { addr: u64, value: u64 },
    Mask { mask: u64, value: u64 },
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instr, Error> {
        let mem_re = Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
        let bitmask_re = Regex::new("mask = ([01X]+)").unwrap();

        if let Some(caps) = mem_re.captures(s) {
            let addr = caps.get(1).unwrap().as_str().parse()?;
            let value = caps.get(2).unwrap().as_str().parse()?;
            return Ok(Instr::Mem { addr, value });
        }

        if let Some(caps) = bitmask_re.captures(s) {
            let pattern = caps.get(1).unwrap().as_str();
            let mut mask = 0;
            let mut value = 0;

            for bit in pattern.chars() {
                mask <<= 1;
                value <<= 1;

                if bit == 'X' {
                    mask |= 1;
                }

                if bit == '1' {
                    value |= 1;
                }
            }

            return Ok(Instr::Mask { mask, value });
        }

        bail!("Invalid instruction");
    }
}
