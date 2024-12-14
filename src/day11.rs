use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn count(n: usize, val: i64, cache: &mut HashMap<(usize, i64), i64>) -> i64 {
    if n == 0 {
        return 1;
    }

    if let Some(c) = cache.get(&(n, val)) {
        return *c;
    }

    let c = if val == 0 {
        count(n - 1, 1, cache)
    } else {
        let num_digits = val.ilog10() + 1;
        if num_digits % 2 == 0 {
            let p = 10i64.pow(num_digits / 2);
            let a = val / p;
            let b = val % p;

            count(n - 1, a, cache) + count(n - 1, b, cache)
        } else {
            count(n - 1, val * 2024, cache)
        }
    };

    cache.insert((n, val), c);

    c
}

#[aoc(day11, part1)]
fn part1(input: &str) -> i64 {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|x| count(25, x.parse().unwrap(), &mut cache))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> i64 {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|x| count(75, x.parse().unwrap(), &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 55312);
    }
}
