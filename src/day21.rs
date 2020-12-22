use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = include_str!("../input/day21");
    let foods = parse(input);

    let ingredient_to_allergen = map_ingredient_to_allergen(&foods);

    println!("Part 1: {}", part1(&foods, &ingredient_to_allergen));
    println!("Part 2: {}", part2(&ingredient_to_allergen));
}

fn part1(foods: &[Food], ingredient_to_allergen: &HashMap<&str, &str>) -> u32 {
    let mut count = 0;

    for f in foods {
        for ingredient in &f.ingredients {
            if !ingredient_to_allergen.contains_key(ingredient) {
                count += 1;
            }
        }
    }

    count
}

fn part2(ingredient_to_allergen: &HashMap<&str, &str>) -> String {
    let mut dangerous_list: Vec<&str> = ingredient_to_allergen.keys().copied().collect();
    dangerous_list.sort_by_key(|k| ingredient_to_allergen[k]);
    dangerous_list.join(",")
}

fn map_ingredient_to_allergen<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, &'a str> {
    let mut remaining_ingredients = HashSet::new();
    let mut remaining_allergens = HashSet::new();

    for f in foods {
        for &ingredient in &f.ingredients {
            remaining_ingredients.insert(ingredient);
        }

        for &allergen in &f.allergens {
            remaining_allergens.insert(allergen);
        }
    }

    let mut ingredient_to_allergen = HashMap::new();

    while !remaining_allergens.is_empty() {
        for allergen in remaining_allergens.clone() {
            let mut possible: HashSet<&str> = remaining_ingredients.clone();

            for f in foods {
                if f.allergens.contains(&allergen) {
                    possible = possible.intersection(&f.ingredients).copied().collect();
                }
            }

            if possible.len() == 1 {
                let ingredient = possible.into_iter().next().unwrap();
                remaining_ingredients.remove(ingredient);
                remaining_allergens.remove(allergen);

                ingredient_to_allergen.insert(ingredient, allergen);
            }
        }
    }

    ingredient_to_allergen
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
