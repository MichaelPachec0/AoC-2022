//! --- Day 3: Rucksack Reorganization ---
//! This application solves Day 3 of advent of code.

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
    let sample = reader("../sample.txt", None).collect::<Vec<String>>();
    let input = reader("../input.txt", None).collect::<Vec<String>>();

    println!("The final score for the sample is {}", part_1(&sample));
    println!("The sum of the priorities for part 1 is {}", part_1(&input));
    println!(
        "With consideration of part 2, the priorities for the sample are {}",
        part_2(&sample)
    );
    println!("And for the input text, {}", part_2(&input));
}

/// Used for testing the application. Using the sample input, that it will return the correct output.
fn part_1(data: &[String]) -> i32 {
    rucksacks(data)
        .into_iter()
        .flat_map(sacks_flat_map)
        .fold(0_i32, |acc, char| sum_chars(acc, *char))
}
///
fn part_2(data: &[String]) -> i32 {
    part_2_compute(data)
        .into_iter()
        .fold(0_i32, |acc, char| sum_chars(acc, *char))
}
///
fn part_2_compute(data: &[String]) -> impl Iterator<Item = &u8> {
    data.chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|i: &String| HashSet::<&u8>::from_iter(i.as_bytes()))
                .collect::<Vec<HashSet<&u8>>>()
        })
        .flat_map(|items| {
            items[0]
                .iter()
                .filter(|char| items[1].contains(**char) && items[2].contains(**char))
                .copied()
                .collect::<Vec<&u8>>()
        })
}

/// Multiple functions are calling this to compute the final priorities, abstract this.
fn sum_chars(acc: i32, char: u8) -> i32 {
    acc + i32::from(char - (if char > 96 { 96 } else { 38 }))
}

/// Abstract the `flat_map` iterator function, since its going to be called from multiple places
fn sacks_flat_map<'args>((first, second): (&'args str, &'args str)) -> Vec<&'args u8> {
    let check: HashSet<&u8> = HashSet::from_iter(second.as_bytes());
    HashSet::<&u8>::from_iter(first.as_bytes())
        .into_iter()
        .filter(|char| check.contains(char))
        .collect::<Vec<&u8>>()
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
) -> impl Iterator<Item = String> + 'args_life {
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
fn rucksacks(data: &[String]) -> impl Iterator<Item = (&str, &str)> {
    data.iter()
        .map(|sack| (&sack[..(sack.len() / 2)], &sack[(sack.len() / 2)..]))
}
