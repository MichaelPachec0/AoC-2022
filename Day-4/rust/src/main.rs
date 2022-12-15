use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sample_input = reader("../sample.txt", None).collect::<Vec<String>>();
    let day_input = reader("../input.txt", None).collect::<Vec<String>>();
    println!(
        "The number of assignments that overlap in the sample data is {}",
        part_1(&sample_input)
    );
    println!(
        "The number of assignments that overlap in the input data is {}",
        part_1(&day_input)
    );
    println!(
        "With consideration for part 2 (find all overlaps), the answer for the sample is {}",
        part_2(&sample_input)
    );
    println!("And for the day input it is {}", part_2(&day_input));
}

fn computation(data: &[String]) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    assignments(data).map(|(first, second)| {
        let first = first.split('-').collect::<Vec<&str>>();
        let second = second.split('-').collect::<Vec<&str>>();
        (
            first[0].parse::<i32>().unwrap(),
            first[1].parse::<i32>().unwrap(),
            second[0].parse::<i32>().unwrap(),
            second[1].parse::<i32>().unwrap(),
        )
    })
}

fn part_1(data: &[String]) -> i32 {
    computation(data)
        .filter(|(omin, omax, cmin, cmax)| {
            overlap_only((omin, omax, cmin, cmax)) || overlap_only((cmin, cmax, omin, omax))
        })
        .count() as i32
}

//This returns true if all
fn overlap_only(
    (overlap_min, overlap_max, check_min, check_max): (&i32, &i32, &i32, &i32),
) -> bool {
    overlap_min <= check_min && check_max <= overlap_max
}

fn part_2(data: &[String]) -> i32 {
    computation(data)
        .filter(|(omin, omax, cmin, cmax)| overlap_all((omin, omax, cmin, cmax)))
        .count() as i32
}

fn overlap_all((overlap_min, overlap_max, check_min, check_max): (&i32, &i32, &i32, &i32)) -> bool {
    overlap_min.max(check_min) <= overlap_max.min(check_max)
}

/// Helper function to return a File Buffer. Used to isolate imperative code from the
/// codebase
fn reader_helper(path: &str) -> BufReader<File> {
    // I am fine with it panicking here, all of the code depends on these lines
    // TODO: might figure out later a better way to refactor this code.
    let input = File::open(path).map_or_else(
        |_| {
            panic!("File {path} cannot be read");
        },
        |x| x,
    );
    BufReader::new(input)
}

/// Main file opening code path. Written as an generic Iterator (that returns a String for every line)
/// so that it can be chained with other methods.
fn reader<'args>(path: &str, pattern: Option<&'args str>) -> impl Iterator<Item = String> + 'args {
    reader_helper(path)
        .lines()
        .into_iter()
        .map(Result::unwrap_or_default)
        // the filter below should not consume the String being passed down, instead use a reference,
        // what does need to be consumed is the pattern variable.
        .filter(move |line: &String| !(line.is_empty() || line.contains(pattern.unwrap_or("//"))))
}

/// Day 4 specific code, splits the string from reader into a tuple of two Strings, one denoting the
/// first elves and seconds elves assignment.
fn assignments(data: &[String]) -> impl Iterator<Item = (&str, &str)> {
    data.iter().map(|sack| {
        let mut elves = sack.split(',');
        //There will always be only two sections, if not we should panic anyways since the rest the
        // program should not function without the formatted input.
        // TODO: Pretty printed panic warning?
        (elves.next().unwrap(), elves.next().unwrap())
    })
}
