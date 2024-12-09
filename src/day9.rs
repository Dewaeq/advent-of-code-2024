use aoc_runner_derive::aoc;

fn parse(input: &str) -> Vec<(i64, usize)> {
    let mut res = vec![];

    for (i, c) in input.chars().enumerate() {
        let val = c.to_digit(10).unwrap();
        let len = val as usize;
        if i % 2 == 0 {
            res.extend(vec![((i / 2) as _, len); len]);
        } else {
            res.extend(vec![(-1, len); len]);
        }
    }

    res
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    let mut map = parse(input);

    let mut i = map.len() - 1;
    // not necessary, but improves performance by 127x (!!)
    let mut search_start = 0;
    loop {
        if map[i].0 == -1 {
            i -= map[i].1;
            continue;
        }

        for j in search_start..i {
            if map[j].0 == -1 {
                map.swap(i, j);
                search_start = j + 1;
                break;
            }
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    map.iter()
        .enumerate()
        .map(|(i, &(val, _))| (i as i64) * val.max(0))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    let mut map = parse(input);

    let mut i = map.len() - 1;
    // not necessary, but improves performance by 6x
    let mut search_start = 0;
    loop {
        if map[i].0 == -1 {
            i -= map[i].1;
            continue;
        }

        let block_size = map[i].1;

        let mut updated_start = false;

        for j in search_start..i {
            if map[j].0 == -1 {
                if !updated_start {
                    search_start = j;
                    updated_start = true;
                }

                let available = map[j].1;

                if available >= block_size {
                    for k in j..j + block_size {
                        map.swap(i - k + j, k);
                    }
                    for k in j + block_size..j + available {
                        map[k].1 -= block_size;
                    }

                    i -= block_size - 1;
                    break;
                }
            }
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    map.iter()
        .enumerate()
        .map(|(i, &(val, _))| (i as i64) * val.max(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST), 2858);
    }
}
