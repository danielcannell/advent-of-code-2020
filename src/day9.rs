pub fn solve() {
    let input: Vec<i64> = include_str!("../input/day9")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[i64]) -> i64 {
    find_first_invalid(input)
}

fn part2(input: &[i64]) -> i64 {
    let num = find_first_invalid(input);

    for start in 0..input.len() {
        for end in start..input.len() {
            let window = &input[start..end];
            let sum: i64 = window.iter().sum();

            // The partial sums form an increasing sequence
            if sum > num {
                break;
            }

            if sum == num {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }

    panic!("No weakness found");
}

fn find_first_invalid(input: &[i64]) -> i64 {
    let mut window = [0; 25];

    window[..25].copy_from_slice(&input[..25]);

    for i in 25..input.len() {
        let num = input[i];

        if !is_valid(num, &window) {
            return num;
        }

        window[i % 25] = num;
    }

    panic!("No invalid number found")
}

fn is_valid(num: i64, window: &[i64]) -> bool {
    for &x in window {
        let y = num - x;

        if x != y && window.contains(&y) {
            return true;
        }
    }

    false
}
