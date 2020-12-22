use std::collections::{HashSet, HashMap};
use regex::Regex;

pub fn solve() {
    let input = include_str!("../input/day21");
    let foods = parse(input);

    let mapping = determine_allergens(&foods);

    println!("Part 1: {}", part1(&foods, &mapping));
    println!("Part 2: {}", part2(&mapping));
}

fn part1(foods: &[Food], mapping: &HashMap<&str, &str>) -> u32 {
    let mut count = 0;

    for f in foods {
        for ingredient in &f.ingredients {
            if !mapping.contains_key(ingredient) {
                count += 1;
            }
        }
    }

    count
}

fn part2(mapping: &HashMap<&str, &str>) -> String {
    let mut dangerous_list: Vec<&str> = mapping.keys().copied().collect();
    dangerous_list.sort_by_key(|k| mapping[k]);
    dangerous_list.join(",")
}

// Return mapping from ingedient to allergen
fn determine_allergens<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, &'a str> {
    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();

    for f in foods {
        for &ingredient in &f.ingredients {
            all_ingredients.insert(ingredient);
        }

        for &allergen in &f.allergens {
            all_allergens.insert(allergen);
        }
    }

    let mut foods = foods.to_vec();
    let mut mapping = HashMap::new();

    for _ in 0..all_allergens.len() {
        for &allergen in &all_allergens {
            let mut possible: HashSet<&str> = all_ingredients.clone();

            for f in &foods {
                if f.allergens.contains(&allergen) {
                    possible = possible.intersection(&f.ingredients).copied().collect();
                }
            }

            if possible.len() == 1 {
                let ingredient = possible.into_iter().next().unwrap();

                mapping.insert(ingredient, allergen);

                for f in &mut foods {
                    f.ingredients.remove(&ingredient);
                    f.allergens.remove(&allergen);
                }
            }
        }
    }

    mapping
}

fn parse(input: &str) -> Vec<Food> {
    let re = Regex::new(r"^([\w ]+) \(contains ([\w, ]+)\)$").unwrap();

    let mut foods = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).expect("Invalid line");

        let ingredients = caps.get(1).unwrap().as_str().split(' ').collect();
        let allergens = caps.get(2).unwrap().as_str().split(", ").collect();
        foods.push(Food {
            ingredients,
            allergens,
        });
    }

    foods
}

#[derive(Clone)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}
