use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

// Answer for part 1 is 71124. For part 2 its 204639

fn main() {
    let start: Instant = Instant::now();

    imperative().expect("Whoops it failed");

    let duration: Duration = start.elapsed();
    println!("Time elapsed is: {duration:?}");
}

fn imperative() -> Result<(), Box<dyn std::error::Error>> {
    let path: &str = "../input.txt";
    let input: File = File::open(path)?;

    let buf = BufReader::new(input);

    let mut cur_cal: i32 = 0;
    let mut top: [i32; 3] = [0; 3];

    for line in buf.lines() {
        if line_handler(line?, &mut cur_cal) {
            for (i, cal) in top.iter_mut().enumerate() {
                if cur_cal > *cal {
                    top[i] = cur_cal;
                    break;
                }
            }
            cur_cal = 0;
        }
    }
    let mut sum = 0;
    for &cal in &top {
        sum += cal;
    }
    println!("The answer for part 1 is {}", top[0]);
    println!("The answer for part 2 is {}", sum);
    Ok(())
}

fn line_handler(line: String, cur: &mut i32) -> bool {
    if line.len() != 0 {
        *cur += line.parse::<i32>().unwrap_or(0);
        false
    } else {
        true
    }
}
