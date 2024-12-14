use aoc_runner_derive::aoc;

use crate::utils::{enumerate_pos, get, read_grid, set, shape, Grid, Point};

fn find_region(pos: Point, grid: &mut Grid<char>) -> (i32, i32, i32) {
    let species = get(grid, pos).unwrap();
    let placeholder = species.to_ascii_lowercase();

    if species == placeholder {
        return (0, 0, 0);
    }

    set(grid, pos, placeholder);

    let mut queue = vec![pos];
    let mut area = 1;
    let mut perimeter = 0;
    let mut num_sides = 0;

    while let Some(p) = queue.pop() {
        let mut v_borders = 0;
        let mut h_borders = 0;

        for dir in Point::orth_dirs() {
            let new_pos = p + dir;
            let neighbour = get(grid, new_pos);

            if neighbour.is_some_and(|s| s == species) {
                area += 1;
                set(grid, new_pos, placeholder);
                queue.push(new_pos);
            } else if neighbour.is_none_or(|s| s != placeholder) {
                perimeter += 1;
                if dir.0 == 0 {
                    v_borders += 1;
                } else {
                    h_borders += 1;
                }
            }
        }

        for dir in Point::diag_dirs() {
            let new_pos = p + dir;
            let neighbour = get(grid, new_pos);

            if neighbour.is_some_and(|s| s.to_ascii_lowercase() != placeholder) {
                let a = get(&grid, new_pos + Point(0, -dir.1));
                let b = get(&grid, new_pos + Point(-dir.0, 0));
                num_sides += (a.is_some_and(|s| s == placeholder)
                    && b.is_some_and(|s| s == placeholder)) as i32;
            }
        }

        num_sides += match (v_borders, h_borders) {
            (2, 2) => 4,
            (1, 2) | (2, 1) => 2,
            (1, 1) => 1,
            _ => 0,
        }
    }

    (area, perimeter, num_sides)
}

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let mut grid = read_grid(input, |_, c| c);
    let mut count = 0;
    for pos in enumerate_pos(shape(&grid)) {
        let (area, perimeter, _) = find_region(pos, &mut grid);
        count += area * perimeter;
    }

    count
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let mut grid = read_grid(input, |_, c| c);
    let mut count = 0;
    for pos in enumerate_pos(shape(&grid)) {
        let (area, _, num_sides) = find_region(pos, &mut grid);
        count += area * num_sides;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST), 1206);
    }
}
