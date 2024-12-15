use aoc_runner_derive::aoc;

use crate::utils::{enumerate, get, read_grid, set, Grid, Point};

fn parse(input: &str) -> (Point, Grid<char>, Vec<Point>) {
    let (grid_parts, move_parts) = input.split_once("\n\n").unwrap();
    let mut start_pos = Point::zero();
    let grid = read_grid(grid_parts, |pos, c| {
        if c == '@' {
            start_pos = pos;
        }
        c
    });
    let moves = move_parts
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Point::NORTH),
            '>' => Some(Point::EAST),
            'v' => Some(Point::SOUTH),
            '<' => Some(Point::WEST),
            _ => None,
        })
        .collect();

    (start_pos, grid, moves)
}

fn do_move(grid: &mut Grid<char>, pos: Point, dir: Point) -> bool {
    let new_pos = pos + dir;
    let cur_val = get(grid, pos).unwrap();
    let next_val = get(&grid, new_pos).unwrap();

    if next_val == '#' {
        return false;
    }

    let success = if next_val == 'O' {
        // part one
        do_move(grid, new_pos, dir)
    } else if next_val == '[' || next_val == ']' {
        // part two
        // vertical moves are tricky
        if dir.0 == 0 {
            let neighbour_pos = if next_val == '[' {
                new_pos + Point::EAST
            } else {
                new_pos + Point::WEST
            };
            do_move(grid, new_pos, dir) && do_move(grid, neighbour_pos, dir)
        } else {
            do_move(grid, new_pos, dir)
        }
    } else {
        assert_eq!(next_val, '.');
        set(grid, pos, '.');
        set(grid, new_pos, cur_val);
        return true;
    };

    if success {
        set(grid, pos, '.');
        set(grid, new_pos, cur_val);
    }

    success
}

fn solve(input: &str, c: char) -> i32 {
    let (mut pos, mut grid, moves) = parse(input);
    for m in moves {
        let mut g = grid.clone();
        let success = do_move(&mut g, pos, m);
        if success {
            grid = g;
            pos = pos + m;
        }
    }

    enumerate(&grid)
        .map(|(pos, &val)| if val == c { pos.0 + 100 * pos.1 } else { 0 })
        .sum()
}

#[aoc(day15, part1)]
fn part1(input: &str) -> i32 {
    solve(input, 'O')
}

#[aoc(day15, part2)]
fn part2(input: &str) -> i32 {
    let input = input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    solve(&input, '[')
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const TEST2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST1), 2028);
        assert_eq!(part1(TEST2), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST2), 9021);
    }
}
