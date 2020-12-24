use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input: Vec<Vec<_>> = include_str!("../input/day24").lines().map(parse).collect();

    let black = create_initial_state(&input);

    println!("Part 1: {}", part1(&black));
    println!("Part 2: {}", part2(&black));
}

fn part1(black: &HashSet<(i32, i32)>) -> usize {
    black.len()
}

fn part2(black: &HashSet<(i32, i32)>) -> usize {
    let mut current = black.clone();
    let mut next = HashSet::new();
    let mut neighbours = HashMap::new();

    let deltas = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

    for _ in 0..100 {
        for &tile in &current {
            for &delta in &deltas {
                let pos = (tile.0 + delta.0, tile.1 + delta.1);
                *neighbours.entry(pos).or_insert(0) += 1;
            }
        }

        for (pos, count) in neighbours.drain() {
            if current.contains(&pos) {
                // Any black tile with zero or more than 2 black tiles
                // immediately adjacent to it is flipped to white.
                if count > 0 && count <= 2 {
                    next.insert(pos);
                }
            } else {
                // Any white tile with exactly 2 black tiles immediately
                // adjacent to it is flipped to black.
                if count == 2 {
                    next.insert(pos);
                }
            }
        }

        std::mem::swap(&mut next, &mut current);
        next.clear();
    }

    current.len()
}

fn create_initial_state(input: &[Vec<(i32, i32)>]) -> HashSet<(i32, i32)> {
    let mut black = HashSet::new();

    for steps in input {
        let mut pos = (0, 0);

        for step in steps {
            pos = (pos.0 + step.0, pos.1 + step.1);
        }

        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }

    black
}

fn parse(s: &str) -> Vec<(i32, i32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("e|se|sw|w|nw|ne").unwrap();
    }

    let mut steps = Vec::new();

    for m in RE.find_iter(s) {
        let step = match m.as_str() {
            "e" => (1, 0),
            "se" => (1, -1),
            "sw" => (0, -1),
            "w" => (-1, 0),
            "nw" => (-1, 1),
            "ne" => (0, 1),
            _ => panic!("Invalid direction"),
        };

        steps.push(step);
    }

    steps
}
