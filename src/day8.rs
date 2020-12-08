use std::str::FromStr;
use std::collections::HashSet;
use anyhow::{anyhow, bail, Error};

pub fn solve() {
    let input: Vec<Instr> = include_str!("../input/day8")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(program: &[Instr]) -> i32 {
    execute(program).1
}

fn part2(program: &[Instr]) -> i32 {
    for i in 0..program.len() {
        let instr = program[i];

        match instr {
            Instr::Jmp(v) => {
                let mut new_program: Vec<Instr> = program.iter().cloned().collect();
                new_program[i] = Instr::Nop(v);
                let (success, acc) = execute(&new_program);

                if success {
                    return acc;
                }
            }

            Instr::Nop(v) => {
                let mut new_program: Vec<Instr> = program.iter().cloned().collect();
                new_program[i] = Instr::Jmp(v);
                let (success, acc) = execute(&new_program);

                if success {
                    return acc;
                }
            }

            Instr::Acc(_) => {
            }
        }
    }

    panic!("No valid programs found");
}

fn execute(program: &[Instr]) -> (bool, i32) {
    let mut pc = 0;
    let mut acc = 0;
    let mut trace = HashSet::new();

    loop {
        if pc == program.len() as i32 {
            return (true, acc);
        }

        if trace.contains(&pc) {
            return (false, acc);
        }

        trace.insert(pc);

        match program[pc as usize] {
            Instr::Jmp(offset) => {
                pc += offset;
            }

            Instr::Acc(value) => {
                acc += value;
                pc += 1;
            }

            Instr::Nop(..) => {
                pc += 1;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let opcode = parts.next().ok_or(anyhow!("Invalid input"))?;
        let value: i32 = parts.next().ok_or(anyhow!("Invalid input"))?.parse()?;

        let instr = match opcode {
            "jmp" => Instr::Jmp(value),
            "acc" => Instr::Acc(value),
            "nop" => Instr::Nop(value),
            _ => bail!("Invalid opcode"),
        };

        Ok(instr)
    }
}