use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

// Answer for part 1 is 71124. For part 2 its 204639


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();
    let path = "../input.txt";
    let input = File::open(path)?;

    let buf = BufReader::new(input);

    let mut cur_cal: i32 = 0;
    let mut max_cal: i32 = 0;


    for line in buf.lines() {
        max_cal = line_handler(line?, &mut cur_cal, max_cal);
    }
    println!("The answer for Day 1 part 1 is {max_cal}");
    let duration: Duration = start.elapsed();
    println!("Time elapsed is: {duration:?}");
    Ok(())
}


fn line_handler(line: String, cur: &mut i32, max: i32) -> i32 {
    if line.len() == 0 {
        let tmp: i32 = *cur;
        *cur = 0;
        if tmp > max { tmp } else { max }
    } else {
        *cur = *cur + line.parse().unwrap_or(0);
        max
    }
}