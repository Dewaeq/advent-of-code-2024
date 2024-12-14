use aoc_runner_derive::aoc;

// X = alpha * A1 + beta * B1
// Y = alpha * A2 + beta * B2
// => (alpha) = M^-1 . (X)
//    (beta )          (Y)
//
//    M^-1 = 1 / det(M) . (B2  -B1)
//                        (-A2  A1)
// => alpha = (B2*X - B1*Y)/det(M)
// => beta = (-A2*X + A1*Y)/det(M)

fn extract(s: &str) -> (i64, i64) {
    let (x, y) = s.split_once(", ").unwrap();
    (x.parse().unwrap(), y[2..].parse().unwrap())
}

fn solve(input: &str, is_part_one: bool) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .array_chunks()
        .map(|[a, b, prize]| {
            let (a1, a2) = extract(&a[12..]);
            let (b1, b2) = extract(&b[12..]);
            let (mut x, mut y) = extract(&prize[9..]);
            if !is_part_one {
                x += 10000000000000;
                y += 10000000000000;
            }

            let det = (a1 * b2 - a2 * b1) as f64;
            if det == 0. {
                return 0;
            }

            let alpha = (b2 * x - b1 * y) as f64 / det;
            let beta = (-a2 * x + a1 * y) as f64 / det;

            if (beta - beta.round()).abs() + (alpha - alpha.round()).abs() > 1.0e-9 {
                return 0;
            }

            (alpha * 3. + beta) as i64
        })
        .sum()
}

#[aoc(day13, part1)]
fn part1(input: &str) -> i64 {
    solve(input, true)
}

#[aoc(day13, part2)]
fn part2(input: &str) -> i64 {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 480);
    }
}
