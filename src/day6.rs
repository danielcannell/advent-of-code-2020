use std::collections::hash_set::HashSet;

pub fn solve() {
    let input = include_str!("../input/day6");

    let mut groups = Vec::new();

    for group_str in input.split("\n\n") {
        let mut group = Vec::new();

        for line in group_str.lines() {
            group.push(line.chars().collect());
        }

        groups.push(group);
    }

    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
}

fn part1(groups: &[Vec<HashSet<char>>]) -> usize {
    let mut count = 0;

    for g in groups {
        let mut qs = HashSet::new();

        for p in g {
            qs = qs.union(p).cloned().collect();
        }

        count += qs.len();
    }

    count
}

fn part2(groups: &[Vec<HashSet<char>>]) -> usize {
    let mut count = 0;

    for g in groups {
        let mut qs: HashSet<char> = ('a'..='z').collect();

        for p in g {
            qs = qs.intersection(p).cloned().collect();
        }

        count += qs.len();
    }

    count
}