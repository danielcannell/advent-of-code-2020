pub fn solve() {
    let input: Vec<u64> = include_str!("../input/day10")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(adaptors: &[u64]) -> u64 {
    let joltages = joltages_from_adaptors(adaptors);

    let mut num_diff_1 = 0;
    let mut num_diff_3 = 0;

    for i in 0..(joltages.len() - 1) {
        match joltages[i + 1] - joltages[i] {
            1 => num_diff_1 += 1,
            3 => num_diff_3 += 1,
            _ => (),
        }
    }

    num_diff_1 * num_diff_3
}

fn part2(adaptors: &[u64]) -> u64 {
    let joltages = joltages_from_adaptors(adaptors);

    let mut num_configs = vec![1];
    for i in 1..joltages.len() {
        let mut ways = 0;

        for j in (0..i).rev() {
            if joltages[i] - joltages[j] > 3 {
                break;
            }

            ways += num_configs[j];
        }

        num_configs.push(ways);
    }

    num_configs[num_configs.len() - 1]
}

fn joltages_from_adaptors(adaptors: &[u64]) -> Vec<u64> {
    let mut joltages = adaptors.to_vec();
    joltages.push(0); // Socket
    joltages.push(joltages.iter().max().unwrap() + 3); // Laptop
    joltages.sort_unstable();
    joltages
}
