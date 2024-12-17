use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

use crate::utils::{get, pop_min, read_grid, Grid, Point};

fn children(pos: Point, dir: Point, grid: &Grid<char>) -> Vec<(i32, (Point, Point))> {
    let mut res = vec![];
    let mut inner_neighbours = |cost, pos, dir| {
        if let Some(val) = get(grid, pos) {
            if val != '#' {
                res.push((cost, (pos, dir)));
            }
        }
    };

    inner_neighbours(1, pos + dir, dir);
    inner_neighbours(1000, pos, dir.rotate_dir(true));
    inner_neighbours(1000, pos, dir.rotate_dir(false));

    res
}

fn dijkstra(
    start: (Point, Point),
    grid: &Grid<char>,
) -> (
    HashMap<(Point, Point), i32>,
    HashMap<(Point, Point), Vec<(Point, Point)>>,
) {
    let mut costs = HashMap::new();
    costs.insert(start, 0);

    let mut parents = HashMap::new();

    let mut queue = vec![(0, start)];

    while let Some((parent_cost, parent)) = pop_min(&mut queue, |x| x.0) {
        for (child_cost, child) in children(parent.0, parent.1, grid) {
            let new_cost = parent_cost + child_cost;

            if let Some(&d) = costs.get(&child) {
                if d > new_cost {
                    costs.insert(child, new_cost);
                    parents.insert(child, vec![parent]);

                    queue.push((new_cost, child));
                } else if d == new_cost {
                    parents.entry(child).and_modify(|v| v.push(parent));
                }
            } else {
                costs.insert(child, new_cost);
                parents.insert(child, vec![parent]);

                queue.push((new_cost, child));
            }
        }
    }

    (costs, parents)
}

fn parse(input: &str) -> (Point, Point, Grid<char>) {
    let mut start_pos = Point::zero();
    let mut end_pos = Point::zero();
    let grid = read_grid(input, |pos, c| {
        if c == 'S' {
            start_pos = pos;
        } else if c == 'E' {
            end_pos = pos;
        }

        c
    });

    (start_pos, end_pos, grid)
}

#[aoc(day16, part1)]
fn part1(input: &str) -> i32 {
    let (start, end, grid) = parse(input);
    let (costs, _) = dijkstra((start, Point::EAST), &grid);

    let mut min_cost = i32::max_value();
    for dir in Point::orth_dirs() {
        if let Some(&cost) = costs.get(&(end, dir)) {
            min_cost = min_cost.min(cost);
        }
    }

    min_cost
}

#[aoc(day16, part2)]
fn part2(input: &str) -> i32 {
    let (start, end, grid) = parse(input);
    let (costs, parents) = dijkstra((start, Point::EAST), &grid);

    let mut min_cost = i32::max_value();
    for dir in Point::orth_dirs() {
        if let Some(&cost) = costs.get(&(end, dir)) {
            min_cost = min_cost.min(cost);
        }
    }

    let mut seen = HashSet::new();

    for dir in Point::orth_dirs() {
        if let Some(&cost) = costs.get(&(end, dir)) {
            if cost == min_cost {
                if let Some(p) = parents.get(&(end, dir)) {
                    let mut queue = p.clone();

                    while let Some(parent) = queue.pop() {
                        if seen.contains(&parent) {
                            continue;
                        }
                        seen.insert(parent);
                        for grand_parent in parents.get(&parent).unwrap_or(&vec![]) {
                            queue.push(*grand_parent);
                        }
                    }
                }
            }
        }
    }

    seen.iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len() as i32
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST1), 7036);
        assert_eq!(part1(TEST2), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST1), 45);
        assert_eq!(part2(TEST2), 64);
    }
}
