use std::collections::HashMap;

pub fn solve() {
    let input: Vec<u32> = include_str!("../input/day15")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[u32]) -> u32 {
    play_game(input, 2020)
}

fn part2(input: &[u32]) -> u32 {
    play_game(input, 30000000)
}

fn play_game(input: &[u32], stop_time: u32) -> u32 {
    let mut history = HashMap::new();

    let mut time = 0;
    let mut delta = 0;
    let mut n = 0;

    for &n in input {
        delta = if history.contains_key(&n) {
            time - history[&n]
        } else {
            0
        };

        history.insert(n, time);
        time += 1;
    }

    while time != stop_time {
        n = delta;

        delta = match history.get(&n) {
            Some(prev_time) => time - prev_time,
            None => 0,
        };

        history.insert(n, time);
        time += 1;
    }

    n
}

#[test]
fn part1_examples() {
    assert_eq!(part1(&[0, 3, 6]), 436);
    assert_eq!(part1(&[1, 3, 2]), 1);
    assert_eq!(part1(&[2, 1, 3]), 10);
    assert_eq!(part1(&[1, 2, 3]), 27);
    assert_eq!(part1(&[2, 3, 1]), 78);
    assert_eq!(part1(&[3, 2, 1]), 438);
    assert_eq!(part1(&[3, 1, 2]), 1836);
}
