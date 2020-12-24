pub fn solve() {
    let input: Vec<u32> = include_str!("../input/day23")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Expected a digit"))
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[u32]) -> u32 {
    let mut ring = Ring::new(input, input.len() as u32);
    ring.play(100);
    ring.part1_result()
}

fn part2(input: &[u32]) -> u64 {
    let mut ring = Ring::new(input, 1_000_000);
    ring.play(10_000_000);
    ring.part2_result()
}

#[derive(Debug)]
struct Ring {
    next: Vec<u32>,
    current: u32,
}

impl Ring {
    fn new(cups: &[u32], limit: u32) -> Ring {
        let mut ring = Ring {
            current: cups[0],
            next: vec![0; limit as usize + 1],
        };

        let elem = |i| cups.get(i as usize).copied().unwrap_or(i + 1);

        for i in 0..limit {
            ring.set_next(elem(i), elem((i + 1) % limit));
        }

        ring
    }

    fn nth(&self, cup: u32, n: u32) -> u32 {
        if n == 0 {
            cup
        } else {
            self.next(self.nth(cup, n - 1))
        }
    }

    fn contains(&self, cup: u32, start: u32, n: u32) -> bool {
        if n == 0 {
            false
        } else if cup == start {
            true
        } else {
            self.contains(cup, self.next(start), n - 1)
        }
    }

    fn next(&self, cup: u32) -> u32 {
        self.next[cup as usize]
    }

    fn set_next(&mut self, cup: u32, next: u32) {
        self.next[cup as usize] = next;
    }

    fn play(&mut self, rounds: u32) {
        for _ in 0..rounds {
            self.play_round();
        }
    }

    fn play_round(&mut self) {
        // Remove a segment of three cups
        let segment = self.next(self.current);
        self.set_next(self.current, self.nth(self.current, 4));

        let mut insert_after = self.current - 1;
        loop {
            if insert_after == 0 {
                insert_after = self.next.len() as u32 - 1;
                continue;
            }

            if self.contains(insert_after, segment, 3) {
                insert_after -= 1;
                continue;
            }

            break;
        }

        self.set_next(self.nth(segment, 2), self.next(insert_after));
        self.set_next(insert_after, segment);

        self.current = self.next(self.current);
    }

    fn part1_result(&self) -> u32 {
        let mut result = 0;
        let mut n = self.next(1);

        while n != 1 {
            result = 10 * result + n;
            n = self.next(n);
        }

        result
    }

    fn part2_result(&self) -> u64 {
        let a = self.next(1);
        let b = self.next(a);
        a as u64 * b as u64
    }
}
