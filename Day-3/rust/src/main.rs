//! --- Day 3: Rucksack Reorganization ---
//! This application solves Day 3 of advent of code.

use std::borrow::ToOwned;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Each rucksack has two large compartments. All items of a given type are meant to go into exactly
// one of the two compartments. The Elf that did the packing failed to follow this rule for
// exactly one item type per rucksack.
//
// The Elves have made a list of all of the items currently in each rucksack (your puzzle input),
// but they need your help finding the errors. Every item type is identified by a single
// lowercase or uppercase letter (that is, a and A refer to different types of items).
// The list of items for each rucksack is given as characters all on a single line.
// A given rucksack always has the same number of items in each of its two compartments,
// so the first half of the characters represent items in the first compartment, while the second
// half of the characters represent items in the second compartment.

// Every rucksack will have an even number of items

fn main() {
    let sacks = rucksacks("../input.txt").collect::<Vec<(String, String)>>();

    println!("The final score for the sample is {}", sample());
    println!("The sum of the priorities for part 1 is {}", part_1(&sacks));
}

/// Used for testing the application. Using the sample input, that it will return the correct output.
fn sample() -> i32 {
    compute_sum("../sample.txt")
}

/// part 1 of the problem
fn part_1(sacks: &[(String, String)]) -> i32 {
    compute_sum_iter(sacks).fold(0_i32, sum_chars)
}

/// Abstract the iterator, this time we return an iterator, so that it can be generically applied to
/// either part 1 or part 2
fn compute_sum_iter(sacks: &[(String, String)]) -> impl Iterator<Item=u8> + '_ {
    sacks.iter()
        .flat_map(sacks_flat_map)
}
/// Multiple functions are calling this to compute the final priorities, abstract this.
fn sum_chars(acc: i32, char: u8) -> i32 {
    acc + i32::from(char - (if char > 96 { 96 } else { 38 }))
}



/// Abstract the `flat_map` iterator, since its going to be called from multiple places
#[allow(clippy::pattern_type_mismatch)]
fn sacks_flat_map((first, second): &(String, String)) -> Vec<u8> {
    let check: HashSet<&u8> = HashSet::from_iter(second.as_bytes());
    HashSet::<&u8>::from_iter(first.as_bytes()).into_iter()
        .filter(|char| check.contains(char))
        .copied()
        .collect::<Vec<u8>>()
}


/// This abstracts the most of the work from the functions sample and part 1
fn compute_sum(path: &str) -> i32 {
    rucksacks(path)
        .flat_map(|tuple| sacks_flat_map(&tuple))
        .fold(0_i32, sum_chars)
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
fn reader<'args_life>(
    path: &str,
    pattern: Option<&'args_life str>,
) -> impl Iterator<Item=String> + 'args_life {
    reader_helper(path)
        .lines()
        .into_iter()
        .map(Result::unwrap_or_default)
        // the filter below should not consume the String being passed down, instead use a reference,
        // what does need to be consumed is the pattern variable.

        .filter(move |line: &String| !(line.is_empty() || line.contains(pattern.unwrap_or("//"))))
}

/// Day 3 specific code, splits the string from reader into a tuple of two Strings, one denoting the
/// first pocket in the rucksack, same with the second.
fn rucksacks(path: &str) -> impl Iterator<Item=(String, String)> + '_ {
    reader(path, None).into_iter().map(|line: String| {
        // Clippy does not know better, integer arithmetic should not be able to be overflowed
        // since we are measuring a size of something.
        #[allow(clippy::integer_division)]
        (
            // Did not want to do this, but i guess i have to.
            // Want to see if i can do Box or Rc, instead so that i dont have to do a clone.
            line.get(..(line.len() / 2)).map_or_else(
                || panic!("Line: {line} panicked when slicing"),
                ToOwned::to_owned,
            ),
            line.get(line.len() / 2..).map_or_else(
                || panic!("Line: {line} panicked when slicing"),
                ToOwned::to_owned,
            ),
        )
    })
}
