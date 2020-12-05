use std::collections::hash_set::HashSet;

pub fn solve() {
    let input: HashSet<u32> = include_str!("../input/day5")
        .lines()
        .map(seat_id_from_pass)
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(seat_ids: &HashSet<u32>) -> u32 {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &HashSet<u32>) -> u32 {
    let first_seat = *seat_ids.iter().min().unwrap();
    let last_seat = *seat_ids.iter().max().unwrap();

    for seat_id in first_seat..last_seat {
        if !seat_ids.contains(&seat_id) {
            return seat_id;
        }
    }

    panic!("No free seats")
}

fn seat_id_from_pass(pass: &str) -> u32 {
    // Binary representation of the seat ID, MSB first
    let seat_id_bin = pass.chars().map(|c| match c {
        'B' | 'R' => 1,
        'F' | 'L' => 0,
        _ => panic!("Invalid boarding pass"),
    });

    seat_id_bin.fold(0, |acc, b| 2 * acc + b)
}

#[test]
fn seat_id_from_pass_on_example() {
    assert_eq!(seat_id_from_pass("BFFFBBFRRR"), 567);
    assert_eq!(seat_id_from_pass("FFFBBBFRRR"), 119);
    assert_eq!(seat_id_from_pass("BBFFBBFRLL"), 820);
}
