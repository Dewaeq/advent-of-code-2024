use aoc_runner_derive::aoc;

use crate::utils::{enumerate, get, read_grid, set, Grid, Point};

fn search(pos: Point, grid: &mut Grid<i32>, is_part_one: bool) -> i32 {
    let val = get(grid, pos).unwrap();

    if val == 9 {
        if is_part_one {
            set(grid, pos, -1);
        }
        return 1;
    }

    let mut count = 0;
    for dir in Point::orth_dirs() {
        let new_pos = pos + dir;
        if let Some(next) = get(grid, new_pos) {
            if next == val + 1 {
                count += search(new_pos, grid, is_part_one);
            }
        }
    }

    count
}

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let grid = read_grid(input, |_, c| c.to_digit(10).unwrap() as i32);

    let mut count = 0;
    for (pos, &val) in enumerate(&grid) {
        if val == 0 {
            count += search(pos, &mut grid.clone(), true);
        }
    }

    count
}

#[aoc(day10, part2)]
fn part2(input: &str) -> i32 {
    let grid = read_grid(input, |_, c| c.to_digit(10).unwrap() as i32);

    let mut count = 0;
    for (pos, &val) in enumerate(&grid) {
        if val == 0 {
            count += search(pos, &mut grid.clone(), false);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST), 81);
    }
}
