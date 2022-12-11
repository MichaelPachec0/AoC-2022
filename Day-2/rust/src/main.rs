use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let start: Instant = Instant::now();
    let input = reader("../input.txt", None).collect::<Vec<String>>();
    let outcomes = outcomes();
    let p1_score = part_1(&outcomes, &input);
    let p1_duration = start.elapsed();
    println!("The score for the player using the guide is {p1_score}");
    println!("Time elapsed for part 1 is {p1_duration:?}");
    let print_duration = start.elapsed();
    let conditions = win_lose();
    let p2_score = part_2(&outcomes, &conditions, &input);
    let p2_duration = start.elapsed() - print_duration;
    println!("The score when taking into account part 2 conditions is {p2_score}");
    println!("Time elapsed for part 2 is {p2_duration:?}");
    let whole_duration = start.elapsed();
    println!("The whole app took {whole_duration:?}");
}

fn reader_helper(path: &str) -> BufReader<File> {
    // I am fine with it panicking here, all of the code depends on these lines
    // TODO: might figure out later a better way to refactor this code.
    let input = File::open(path).unwrap();
    return BufReader::new(input);
}
fn outcomes() -> HashMap<String, i32> {
    HashMap::from_iter(
        _test_map("outcomes.txt", ":")
            .into_iter()
            .map(|(outcome, score)| {
                (
                    outcome,
                    i32::from_str_radix(score.as_str(), 10).unwrap_or(0),
                )
            })
            .collect::<Vec<(String, i32)>>(),
    )
}
fn win_lose() -> HashMap<String, String> {
    HashMap::from_iter(
        _test_map("part2.txt", ":")
            .into_iter()
            .collect::<Vec<(String, String)>>(),
    )
}
// Need to set lifetimes since the compiler needs to be certain that the variable references being
// sent are valid until the end of the function's execution. This is because we do a move ownership
fn _test_map<'a>(path: &'a str, delimiter: &'a str) -> impl Iterator<Item = (String, String)> + 'a {
    reader(path, None)
        .into_iter()
        // in order for the closure to have access to delimiter, we need a move here, even if its a
        // a &str
        .map(move |line: String| {
            let mut vec = line.split(delimiter).map(String::from);
            // TODO: Do some error handling here.
            (
                vec.next().unwrap_or_default(),
                vec.next().unwrap_or_default(),
            )
        })
}

fn part_1(choices: &HashMap<String, i32>, input: &Vec<String>) -> i32 {
    // The score for a single round is the score for the shape you selected
    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    // plus the score for the outcome of the round
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).
    // already precompiled all outcomes
    input.iter().fold(0, |acc, line| acc + choices[line])
}

fn reader<'a>(path: &'a str, pattern: Option<&'a str>) -> impl Iterator<Item = String> + 'a {
    reader_helper(path)
        .lines()
        .into_iter()
        .map(|line: Result<String, _>| line.unwrap_or_default())
        // the filter below should not consume the String being passed down, instead use a reference,
        // what does need to be consumed is the pattern variable.
        .filter(move |line: &String| !(line.is_empty() || line.contains(pattern.unwrap_or("//"))))
}

fn part_2(
    outcomes: &HashMap<String, i32>,
    condition: &HashMap<String, String>,
    input: &Vec<String>,
) -> i32 {
    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
    input
        .iter()
        .map(|line| {
            let enemy = line.split(" ").nth(0).unwrap_or("A");
            let choice = condition[line].as_str();
            format!("{enemy} {choice}")
        })
        .fold(0, |acc, line| acc + outcomes[&line])
}
