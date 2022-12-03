use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start: Instant = Instant::now();
    let path = "../input.txt";
    let input = File::open(path)?;

    let buf = BufReader::new(input);

    let mut cur_cal = 0;
    let mut max_cal: i32 = 0;

    let debug = false;


    let mut line_handler = |line: String| {
        if line.len() == 0 {
            if debug {println!("CURRENT: CAL {cur_cal}" )};
            if cur_cal > max_cal {
                max_cal = cur_cal
            }
            cur_cal = 0;
        } else {
            cur_cal += line.parse().unwrap_or(0);
        }
    };

    for line in buf.lines() {
        line_handler(line?);
    }
    println!("The answer for Day 1 part 1 is {max_cal}");
    let duration: Duration  = start.elapsed();
    println!("Time elapsed is: {duration:?}");
    Ok(())
}

