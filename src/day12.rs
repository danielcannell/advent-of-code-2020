use anyhow::{bail, Error};
use std::str::FromStr;

pub fn solve() {
    let input: Vec<Move> = include_str!("../input/day12")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(moves: &[Move]) -> i32 {
    let mut p = (0, 0);
    let mut wp = (1, 0);

    for m in moves {
        p = move_abs(m, p);
        wp = rotate(m, wp);
        p = move_rel(m, p, wp);
    }

    p.0.abs() + p.1.abs()
}

fn part2(moves: &[Move]) -> i32 {
    let mut p = (0, 0);
    let mut wp = (10, 1);

    for m in moves {
        wp = move_abs(m, wp);
        wp = rotate(m, wp);
        p = move_rel(m, p, wp);
    }

    p.0.abs() + p.1.abs()
}

fn move_abs(m: &Move, p: (i32, i32)) -> (i32, i32) {
    match m {
        Move::North(amt) => (p.0, p.1 + amt),
        Move::East(amt) => (p.0 + amt, p.1),
        _ => p,
    }
}

fn rotate(m: &Move, p: (i32, i32)) -> (i32, i32) {
    match m {
        Move::Left(amt) => match amt {
            90 => (-p.1, p.0),
            180 => (-p.0, -p.1),
            270 => (p.1, -p.0),
            _ => panic!("Invalid angle"),
        },
        _ => p,
    }
}

fn move_rel(m: &Move, p: (i32, i32), wp: (i32, i32)) -> (i32, i32) {
    match m {
        Move::Forward(amt) => (p.0 + amt * wp.0, p.1 + amt * wp.1),
        _ => p,
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    North(i32),
    East(i32),
    Left(i32),
    Forward(i32),
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Move, Error> {
        if s.is_empty() {
            bail!("Empty move");
        }

        let amt = s[1..].parse()?;

        Ok(match s.chars().next().unwrap() {
            'N' => Move::North(amt),
            'S' => Move::North(-amt),
            'E' => Move::East(amt),
            'W' => Move::East(-amt),
            'L' => Move::Left(amt),
            'R' => Move::Left(360 - amt),
            'F' => Move::Forward(amt),
            _ => bail!("Invalid direction"),
        })
    }
}

#[test]
fn test_part1_on_example() {
    let input: Vec<Move> = "F10\nN3\nF7\nR90\nF11"
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(part1(&input), 25);
}

#[test]
fn test_part2_on_example() {
    let input: Vec<Move> = "F10\nN3\nF7\nR90\nF11"
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(part2(&input), 286);
}
