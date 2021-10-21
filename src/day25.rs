pub fn solve() {
    let input: Vec<u64> = include_str!("../input/day25")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &[u64]) -> u64 {
    transform(input[0], crack(input[1]))
}

fn crack(public_key: u64) -> u64 {
    let mut value = 1;
    let mut private_key = 0;
    let subject = 7;

    while value != public_key {
        value *= subject;
        value %= 20201227;
        private_key += 1;
    }

    private_key
}

fn transform(subject: u64, key: u64) -> u64 {
    let mut value = 1;

    for _ in 0..key {
        value *= subject;
        value %= 20201227;
    }

    value
}

#[test]
fn test_part1_example() {
    assert_eq!(crack(5764801), 8);
    assert_eq!(crack(17807724), 11);
    assert_eq!(transform(5764801, 11), 14897079);
    assert_eq!(transform(17807724, 8), 14897079);
    assert_eq!(part1(&[5764801, 17807724]), 14897079);
}
