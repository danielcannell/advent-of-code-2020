use fromstr_derive::FromStr;

#[derive(Debug, FromStr)]
#[regex(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")]
struct Password {
    // Password policy
    min: usize,
    max: usize,
    letter: char,

    // The password
    password: String,
}

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
