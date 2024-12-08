use crate::utils::{colinear, dist_squared, enumerate, slope, Point};
use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn count(input: &str, is_part_two: bool) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let antennas: HashMap<char, Vec<Point>> =
        enumerate(&grid).fold(HashMap::new(), |mut map, (pos, &val)| {
            if val != '.' {
                map.entry(val)
                    .and_modify(|v| v.push(pos))
                    .or_insert(vec![pos]);
            }
            map
        });

    let mut count = 0;

    'outer: for (pos, _) in enumerate(&grid) {
        for (_, props) in &antennas {
            for &antenna1 in props {
                for &antenna2 in props {
                    if antenna1 == antenna2 {
                        continue;
                    }

                    if is_part_two {
                        if colinear(pos, antenna1, antenna2) || pos == antenna1 || pos == antenna2 {
                            count += 1;
                            continue 'outer;
                        }
                    } else {
                        let dist1 = dist_squared(pos, antenna1);
                        let dist2 = dist_squared(pos, antenna2);
                        let dir1 = slope(pos, antenna1);
                        let dir2 = slope(pos, antenna2);

                        if dist1 == 0 || dist2 == 0 {
                            continue;
                        }

                        if dist1 == 4 * dist2 && dir1 == dir2 {
                            count += 1;
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    count
}

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    count(input, false)
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    count(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST), 34);
    }
}
