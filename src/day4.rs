use aoc_runner_derive::aoc;

fn get(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> Option<char> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    grid.get(pos.1 as usize)
        .map(|row| row.get(pos.0 as usize).cloned())
        .flatten()
}

fn search(grid: &Vec<Vec<char>>, target: char, pos: (i32, i32), dir: (i32, i32)) -> i32 {
    match get(grid, pos) {
        Some(c) if c == target => {
            let next_c = match c {
                'X' => 'M',
                'M' => 'A',
                'A' => 'S',
                'S' => return 1,
                _ => unreachable!(),
            };

            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            search(grid, next_c, new_pos, dir)
        }
        _ => 0,
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut count = 0;

    for y in 0..rows {
        for x in 0..cols {
            for dir in [
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ] {
                count += search(&grid, 'X', (x, y), dir);
            }
        }
    }

    count
}

fn is_x_mas(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    if get(grid, pos).is_none_or(|c| c != 'A') {
        return false;
    }

    let (x, y) = (pos.0 as usize, pos.1 as usize);
    // M . .     S . .
    // . A . or  . A .
    // . . S     . . M
    if !(grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S'
        || grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
    {
        return false;
    }

    // . . M     . . S
    // . A . or  . A .
    // S . .     M . .
    if !(grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S'
        || grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
    {
        return false;
    }

    true
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut count = 0;

    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            count += is_x_mas(&grid, (x, y)) as i32;
        }
    }

    count
}
