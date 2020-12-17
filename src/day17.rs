use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

pub fn solve() {
    let input = include_str!("../input/day17");

    let mut active = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, s) in l.chars().enumerate() {
            let pos = (x as i32, y as i32);

            if s == '#' {
                active.insert(pos);
            }
        }
    }

    println!("Part 1: {}", part1(&active));
    println!("Part 2: {}", part2(&active));
}

fn part1(active: &HashSet<(i32, i32)>) -> usize {
    simulate::<Point3D>(active)
}

fn part2(active: &HashSet<(i32, i32)>) -> usize {
    simulate::<Point4D>(active)
}

fn simulate<P: Point>(active: &HashSet<(i32, i32)>) -> usize {
    let mut active: HashSet<P> = active.iter().map(P::from_2d).collect();

    let deltas = P::deltas();

    for _ in 0..6 {
        let mut neighbours = HashMap::new();

        for &p in &active {
            for &dp in &deltas {
                *neighbours.entry(p + dp).or_insert(0u32) += 1;
            }
        }

        let mut new = HashSet::new();

        for (p, num_neighbours) in neighbours {
            if num_neighbours >= 2 && num_neighbours <= 3 && active.contains(&p) {
                new.insert(p);
            }

            if num_neighbours == 3 && !active.contains(&p) {
                new.insert(p);
            }
        }

        active = new;
    }

    active.len()
}

trait Point: Sized + Add<Output = Self> + Hash + Eq + Copy {
    fn from_2d(p: &(i32, i32)) -> Self;
    fn deltas() -> Vec<Self>;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point3D(i32, i32, i32);

impl Point for Point3D {
    fn from_2d(p: &(i32, i32)) -> Self {
        Point3D(p.0, p.1, 0)
    }

    fn deltas() -> Vec<Self> {
        let mut d = Vec::new();

        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    if x != 0 || y != 0 || z != 0 {
                        d.push(Point3D(x, y, z));
                    }
                }
            }
        }

        d
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Point3D {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point4D(i32, i32, i32, i32);

impl Point for Point4D {
    fn from_2d(p: &(i32, i32)) -> Self {
        Point4D(p.0, p.1, 0, 0)
    }

    fn deltas() -> Vec<Self> {
        let mut d = Vec::new();

        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    for &w in &[-1, 0, 1] {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            d.push(Point4D(x, y, z, w));
                        }
                    }
                }
            }
        }

        d
    }
}

impl Add<Point4D> for Point4D {
    type Output = Point4D;

    fn add(self, rhs: Point4D) -> Point4D {
        Point4D(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}
