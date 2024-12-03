use aoc_runner_derive::aoc;

fn count(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|x| x.extract().1)
        .fold(0, |acc, [a, b]| {
            acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    count(input)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    // remove whitespace
    let input = regex::Regex::new("[[:space:]]")
        .unwrap()
        .replace_all(input, "");
    // the regex crate sadly does not support look ahead
    //let re = fancy_regex::Regex::new("don't(.*?do(?!n't)|.*don't(?!.*do).*$)").unwrap();
    let re = regex::Regex::new("don't(.*?do([^n])|.*don't.*$)").unwrap();
    let input = re.replace_all(&input, "");

    count(&input)
}
