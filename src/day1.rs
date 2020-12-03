pub fn solve() {
    let input: Vec<i32> = include_str!("../input/day1")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[i32]) -> i32 {
    for (i, first) in input.iter().enumerate() {
        for second in &input[i + 1..] {
            if first + second == 2020 {
                return first * second;
            }
        }
    }

    panic!("No answer found")
}

fn part2(input: &[i32]) -> i32 {
    for (i, first) in input.iter().enumerate() {
        for (j, second) in input[i + 1..].iter().enumerate() {
            for third in &input[i + j + 1..] {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }

    panic!("No answer found")
}
