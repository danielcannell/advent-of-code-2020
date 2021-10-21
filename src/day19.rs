use anyhow::{bail, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../input/day19");

    let (messages, rules) = parse(input);

    println!("Part 1: {}", part1(&rules, &messages));
    println!("Part 2: {}", part2(&rules, &messages));
}

fn part1(rules: &HashMap<u32, Rule>, messages: &[&str]) -> u32 {
    let mut count = 0;

    for &m in messages {
        if is_match(m, rules) {
            count += 1;
        }
    }

    count
}

fn part2(rules: &HashMap<u32, Rule>, messages: &[&str]) -> u32 {
    let mut count = 0;

    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    //
    // => 0: 42{n} 31{m}   where n > m > 0

    for &message in messages {
        let message: Vec<char> = message.chars().collect();
        let rule42 = &rules[&42];
        let rule31 = &rules[&31];

        let mut pos = 0;

        for n in 1.. {
            if !is_match_impl(&message, rule42, &mut pos, rules) {
                break;
            }

            let mut pos = pos;

            for _ in 1..n {
                if !is_match_impl(&message, rule31, &mut pos, rules) {
                    break;
                }

                if pos == message.len() {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse(input: &str) -> (Vec<&str>, HashMap<u32, Rule>) {
    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let messages_str = sections.next().unwrap();

    let mut rules = HashMap::new();
    let messages: Vec<&str> = messages_str.trim().split('\n').collect();

    for line in rules_str.lines() {
        let mut parts = line.split(':');
        let rule_id: u32 = parts.next().unwrap().parse().unwrap();
        let rule: Rule = parts.next().unwrap().parse().unwrap();

        rules.insert(rule_id, rule);
    }

    (messages, rules)
}

fn is_match(message: &str, rules: &HashMap<u32, Rule>) -> bool {
    let message: Vec<char> = message.chars().collect();
    let rule = &rules[&0];
    let mut pos = 0;

    let m = is_match_impl(&message, rule, &mut pos, rules);

    m && pos == message.len()
}

fn is_match_impl(
    message: &[char],
    rule: &Rule,
    pos: &mut usize,
    rules: &HashMap<u32, Rule>,
) -> bool {
    if *pos >= message.len() {
        return false;
    }

    let start_pos = *pos;

    let m = match rule {
        Rule::Literal(lit) => {
            *pos += 1;
            message[*pos - 1] == *lit
        }

        Rule::Ref(r) => is_match_impl(message, &rules[r], pos, rules),

        Rule::Concatenation(lhs, rhs) => {
            let m1 = is_match_impl(message, lhs, pos, rules);
            let m2 = is_match_impl(message, rhs, pos, rules);
            m1 && m2
        }

        Rule::Alternation(lhs, rhs) => {
            is_match_impl(message, lhs, pos, rules) || is_match_impl(message, rhs, pos, rules)
        }
    };

    if !m {
        // Backtrack
        *pos = start_pos;
    }

    m
}

#[derive(Debug)]
enum Rule {
    Literal(char),
    Ref(u32),
    Alternation(Box<Rule>, Box<Rule>),
    Concatenation(Box<Rule>, Box<Rule>),
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Rule, Error> {
        lazy_static! {
            static ref LITERAL_RE: Regex = Regex::new(r#"^"(a|b)"$"#).unwrap();
            static ref REF_RE: Regex = Regex::new(r"^(\d+)$").unwrap();
            static ref CONCAT_RE: Regex = Regex::new(r"^(\d+) (\d+)$").unwrap();
            static ref TRP_CONCAT_RE: Regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
            static ref ALTERN_RE: Regex = Regex::new(r"^(\d+) \| (\d+)$").unwrap();
            static ref DBL_ALTERN_RE: Regex = Regex::new(r"^(\d+) (\d+) \| (\d+) (\d+)$").unwrap();
        }

        let s = s.trim();

        if let Some(caps) = LITERAL_RE.captures(s) {
            let lit = caps.get(1).unwrap().as_str().parse().unwrap();
            return Ok(Rule::Literal(lit));
        }

        if let Some(caps) = REF_RE.captures(s) {
            let r = caps.get(1).unwrap().as_str().parse().unwrap();
            return Ok(Rule::Ref(r));
        }

        if let Some(caps) = CONCAT_RE.captures(s) {
            let a = caps.get(1).unwrap().as_str().parse().unwrap();
            let b = caps.get(2).unwrap().as_str().parse().unwrap();
            return Ok(Rule::Concatenation(
                Box::new(Rule::Ref(a)),
                Box::new(Rule::Ref(b)),
            ));
        }

        if let Some(caps) = TRP_CONCAT_RE.captures(s) {
            let a = caps.get(1).unwrap().as_str().parse().unwrap();
            let b = caps.get(2).unwrap().as_str().parse().unwrap();
            let c = caps.get(3).unwrap().as_str().parse().unwrap();
            let lhs = Rule::Concatenation(Box::new(Rule::Ref(a)), Box::new(Rule::Ref(b)));
            return Ok(Rule::Concatenation(Box::new(lhs), Box::new(Rule::Ref(c))));
        }

        if let Some(caps) = ALTERN_RE.captures(s) {
            let a = caps.get(1).unwrap().as_str().parse().unwrap();
            let b = caps.get(2).unwrap().as_str().parse().unwrap();
            return Ok(Rule::Alternation(
                Box::new(Rule::Ref(a)),
                Box::new(Rule::Ref(b)),
            ));
        }

        if let Some(caps) = DBL_ALTERN_RE.captures(s) {
            let a = caps.get(1).unwrap().as_str().parse().unwrap();
            let b = caps.get(2).unwrap().as_str().parse().unwrap();
            let c = caps.get(3).unwrap().as_str().parse().unwrap();
            let d = caps.get(4).unwrap().as_str().parse().unwrap();

            let lhs = Rule::Concatenation(Box::new(Rule::Ref(a)), Box::new(Rule::Ref(b)));
            let rhs = Rule::Concatenation(Box::new(Rule::Ref(c)), Box::new(Rule::Ref(d)));
            return Ok(Rule::Alternation(Box::new(lhs), Box::new(rhs)));
        }

        bail!("Invalid rule");
    }
}

#[test]
fn part1_example() {
    let input = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb\n";
    let (messages, rules) = parse(input);
    let result = part1(&rules, &messages);
    assert_eq!(result, 2);
}
