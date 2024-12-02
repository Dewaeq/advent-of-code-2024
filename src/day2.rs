use aoc_runner_derive::aoc;

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
        count += 1;
    }

    count
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    todo!()
}
