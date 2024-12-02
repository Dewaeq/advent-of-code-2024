use aoc_runner_derive::aoc;

#[aoc(day2, part1, naive)]
fn part1_naive(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>()
        })
        .map(|x| is_safe(&x) as i32)
        .sum()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let bytes = input.as_bytes();

    let mut count = 0;

    let mut is_safe = true;
    let mut is_descending = None;
    let mut level = 0;
    let mut prev = 0i32;

    for &b in bytes {
        // space/newline means level is fully read
        if is_safe && (b == 10 || b == 32) {
            match (is_descending, prev) {
                (None, x) if x != 0 => is_descending = Some(level < x),
                _ => (),
            }
            if prev != 0 {
                let diff = (level - prev).abs();
                if diff > 3 || diff == 0 || is_descending.is_some_and(|desc| (level < prev) != desc)
                {
                    is_safe = false;
                }
            }

            (prev, level) = (level, 0);
        }
        // newline
        if b == 10 {
            count += is_safe as i32;
            is_safe = true;
            level = 0;
            prev = 0;
            is_descending = None;
        }
        // non-digit
        if b < 48 {
            continue;
        }

        level *= 10;
        level += (b & 0xf) as i32;
    }

    if is_descending.is_some() {
        count += is_safe as i32;
    }

    count
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let bytes = input.as_bytes();

    let mut count = 0;

    let mut report = vec![];
    let mut is_safe = true;
    let mut is_descending = None;
    let mut level = 0;
    let mut prev = 0i32;

    for &b in bytes {
        // space/newline means level is fully read
        if b == 10 || b == 32 {
            match (is_descending, prev) {
                (None, x) if x != 0 => is_descending = Some(level < x),
                _ => (),
            }

            if prev != 0 {
                let diff = (level - prev).abs();
                if diff > 3 || diff == 0 || is_descending.is_some_and(|desc| (level < prev) != desc)
                {
                    is_safe = false;
                }
            }

            report.push(level);
            (prev, level) = (level, 0);
        }
        // newline
        if b == 10 {
            if is_safe {
                count += 1;
            } else {
                count += test(report) as i32;
            }
            is_safe = true;
            level = 0;
            prev = 0;
            is_descending = None;
            report = vec![];
        }
        // non-digit
        if b < 48 {
            continue;
        }

        level *= 10;
        level += (b & 0xf) as i32;
    }

    if is_descending.is_some() {
        if is_safe {
            count += 1;
        } else {
            count += test(report) as i32;
        }
    }

    count
}

fn test(mut l: Vec<i32>) -> bool {
    for skip in 0..(l.len()) {
        let removed = l.remove(skip);

        if is_safe(&l) {
            return true;
        }

        l.insert(skip, removed);
    }

    false
}

fn is_safe(l: &Vec<i32>) -> bool {
    let mut safe = true;
    let sign = (l[0] - l[1]).signum();

    for (idx, x) in l.iter().enumerate() {
        if idx == l.len() - 1 {
            continue;
        }
        let diff = x - l[idx + 1];
        if diff.signum() != sign || diff.abs() > 3 || diff.abs() < 1 {
            safe = false;
            break;
        }
    }
    safe
}
