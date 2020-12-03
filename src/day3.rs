use std::collections::hash_set::HashSet;
use std::str::FromStr;

struct Map {
    width: usize,
    height: usize,

    // (x, y) coordinates of trees in the map
    trees: HashSet<(usize, usize)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Map, ()> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut trees = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    trees.insert((x, y));
                }
            }
        }

        Ok(Map {
            width,
            height,
            trees,
        })
    }
}

pub fn solve() {
    let map = include_str!("../input/day3").parse().unwrap();

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &Map) -> u64 {
    walk(map, 3, 1)
}

fn part2(map: &Map) -> u64 {
    walk(map, 1, 1) * walk(map, 3, 1) * walk(map, 5, 1) * walk(map, 7, 1) * walk(map, 1, 2)
}

fn walk(map: &Map, across: usize, down: usize) -> u64 {
    let mut trees = 0;

    for y in (0..map.height).step_by(down) {
        let x = (y * across / down) % map.width;

        if map.trees.contains(&(x, y)) {
            trees += 1;
        }
    }

    trees
}
