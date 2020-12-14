pub fn solve() {
    let input = include_str!("../input/day13");
    let earliest: u64 = input.lines().nth(0).unwrap().parse().unwrap();
    let busses: Vec<Option<u64>> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();

    println!("Part 1: {}", part1(earliest, &busses));
    println!("Part 2: {}", part2(&busses));
}

fn part1(earliest: u64, busses: &[Option<u64>]) -> u64 {
    let mut first_bus_t = earliest + 100000;
    let mut first_bus_id = 0;

    for &b in busses {
        if let Some(b) = b {
            let offset = earliest % b;
            let t = if offset == 0 {
                earliest
            } else {
                earliest + b - offset
            };

            if t < first_bus_t {
                first_bus_t = t;
                first_bus_id = b;
            }
        }
    }

    let wait = first_bus_t - earliest;

    wait * first_bus_id
}

fn part2(busses: &[Option<u64>]) -> u64 {
    let mut moduli = Vec::new();
    let mut remainders = Vec::new();

    for (i, &b) in busses.iter().enumerate() {
        if let Some(b) = b {
            moduli.push(b);
            remainders.push((10 * b - i as u64) % b);
        }
    }

    let mut x = 0;
    let mut step = 1;

    for (&m, &r) in moduli.iter().zip(&remainders) {
        while x % m != r {
            x += step;
        }

        step *= m;
    }

    x
}
