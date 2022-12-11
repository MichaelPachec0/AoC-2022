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
    println!("Hello, world!");
}

fn reader_helper(path: &str) -> BufReader<File> {
    // I am fine with it panicking here, all of the code depends on these lines
    // TODO: might figure out later a better way to refactor this code.
    let input = File::open(path).unwrap();
    BufReader::new(input)
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

fn rucksacks(path: &str) -> impl Iterator<Item = (String, String)> + '_ {
    reader(path, None).into_iter().map(|line: String| {
        (
            // Did not want to do this, but i guess i have to.
            // Want to see if i can do Box or Rc, instead so that i dont have to do a clone.
            line[..((line.len() / 2) - 1)].to_owned(),
            line[line.len() / 2..].to_owned(),
        )
    })
}
