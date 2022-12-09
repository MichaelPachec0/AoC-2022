use std::{fs::File,
          io::{BufRead, BufReader},
          time::Instant,
          collections::HashMap
};



// The score for a single round is the score for the shape you selected
// (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round
// (0 if you lost, 3 if the round was a draw, and 6 if you won).


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut outcomes = HashMap::new();

    let start: Instant = Instant::now();
    let path = "outcomes.txt";
    let input = File::open(path)?;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        let line = line.unwrap_or(String::from("//"));
        if line.contains("//") {
            continue;
        }
        // TODO: Need to figure out if i can simplify this ugly code
        let outcome = line.split(":").map(|i| String::from(i)).collect::<Vec<String>>();
        outcomes.insert(outcome[0].clone(), i32::from_str_radix(outcome[1].clone().as_str(), 10).unwrap_or(0));
    }
    let path = "../input.txt";
    let input = File::open(path)?;

    let buf = BufReader::new(input);
    let mut score: i32 = 0;

    for line in buf.lines() {
        let line = line.unwrap_or(String::from(""));
        // let outcome = line.split(" ").collect::<Vec<&str>>();
        if line.is_empty() {
            continue;
        }
        score += outcomes[line.as_str()];
    }
    let duration = start.elapsed();
    println!("The scokire for the player using the guide is {score}");
    println!("Time elapsed {duration:?}");
    Ok(())
}