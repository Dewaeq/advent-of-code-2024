use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn parse_rules(s: &str) -> HashMap<i32, Vec<i32>> {
    let mut preceders = HashMap::<i32, Vec<i32>>::new();
    for line in s.lines() {
        let (a, b) = line.split_once("|").unwrap();
        let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
        preceders
            .entry(a)
            .and_modify(|v| v.push(b))
            .or_insert(vec![b]);
    }

    preceders
}

fn parse_updates(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| line.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

fn is_valid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, x) in update.iter().enumerate() {
        if let Some(r) = rules.get(x) {
            for y in &update[..i] {
                if r.contains(y) {
                    return false;
                }
            }
        }
    }

    true
}

fn sort(mut update: Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let len = update.len();

    for i in 0..len {
        let x = update[i];
        if let Some(r) = rules.get(&x) {
            for j in 0..i {
                let y = update[j];
                if r.contains(&y) {
                    update.swap(i, j);
                }
            }
        }
    }

    update
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    updates
        .iter()
        .map(|update| update[update.len() / 2] * (is_valid(&update, &rules) as i32))
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    updates
        .into_iter()
        .filter(|update| !is_valid(update, &rules))
        .map(|update| sort(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}
