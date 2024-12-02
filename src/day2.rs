use aoc_runner_derive::aoc;

fn parse(s: &str) -> i32 {
    s.bytes().fold(0, |a, c| a * 10 + (c - 30) as i32)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut count = 0;
    'outer: for line in input.lines() {
        let mut prev = 0;
        let mut desc = false;
        let mut add = true;

        for (i, entry) in line.split(" ").enumerate() {
            // let level = parse(entry);
            let level = entry.parse::<i32>().unwrap();
            if i == 1 {
                desc = level < prev;
            }
            if i != 0 {
                let diff = (level - prev).abs();
                if diff > 3 || diff == 0 || ((level < prev) != desc) {
                    // continue 'outer;
                    add = false;
                }
            }

            prev = level;
        }
        count += add as i32;
    }

    count
}
#[aoc(day2, part1, cont)]
pub fn part1_cont(input: &str) -> i32 {
    let mut count = 0;
    'outer: for line in input.lines() {
        let mut prev = 0;
        let mut desc = false;

        for (i, entry) in line.split(" ").enumerate() {
            // let level = parse(entry);
            let level = entry.parse::<i32>().unwrap();
            if i == 1 {
                desc = level < prev;
            }
            if i != 0 {
                let diff = (level - prev).abs();
                if diff > 3 || diff == 0 || ((level < prev) != desc) {
                    continue 'outer;
                }
            }

            prev = level;
        }
        count += 1;
    }

    count
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*     #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    } */
}
