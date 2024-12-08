use std::collections::VecDeque;

use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<(i64, VecDeque<i64>)> {
    input
        .lines()
        .map(|line| {
            let (val, parts) = line.split_once(": ").unwrap();
            let val = val.parse().unwrap();
            let parts = parts
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            (val, parts)
        })
        .collect()
}

fn valid(val: i64, target: i64, mut parts: VecDeque<i64>, is_part_two: bool) -> bool {
    if let Some(part) = parts.pop_front() {
        if val == -1 {
            valid(part, target, parts.clone(), is_part_two)
        } else {
            valid(val + part, target, parts.clone(), is_part_two)
                || valid(val * part, target, parts.clone(), is_part_two)
                || (is_part_two && valid(concat(val, part), target, parts.clone(), is_part_two))
        }
    } else {
        val == target
    }
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    let input = parse(input);
    input
        .into_iter()
        .map(|(target, parts)| {
            if valid(-1, target, parts, false) {
                target
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i64 {
    let input = parse(input);
    input
        .into_iter()
        .map(|(target, parts)| {
            if valid(-1, target, parts, true) {
                target
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST), 11387);
    }
}
