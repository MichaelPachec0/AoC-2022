use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
// use itertools::Itertools;
// use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_p1 = part_1("../sample.txt")?;
    println!("{:?}", sample_p1);
    // let input_p1 = part_1("../input.txt")?;

    // println!("{:?}", input_p1);
    Ok(())
}

fn part_1(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input = reader(path)?;
    let raw_data = input.split("\n\n").collect::<Vec<&str>>();
    // println!("RAW DATA: {:?}", raw_data);
    let (data, mut data_flat) = fmt_data(raw_data[0]);
    // &Vec -> Vec -> &str
    // println!("{:?}", data);
    println!("{:?}", data_flat);
    for (i, (num, start, end)) in fmt_ins(raw_data[1]).enumerate() {
        println!("INS: LINE: {i} MOVE: {} FROM S: {} TO E: {} STR: {} ACTUAL: MOVE {num} FROM {start} TO {end}", num, start-1, end-1, data_flat[(start-1) as usize]);
        print!("MOVING: ");
        for i in 0..num {
            let sub_i = TryInto::<usize>::try_into(start)? - 1;
            let add_i = TryInto::<usize>::try_into(end)? - 1;
            // if data_flat.len() < num as usize {
            //     panic!("Not enough characters in place!")
            // }
            let insert = data_flat[sub_i].pop().ok_or(format!(
                "No more chars! N:{} S:{} E:{} I:{} ST:{:?} LOC:{:?} INS: MOVE {} FROM {} TO {}",
                num, sub_i, add_i, i, data_flat, data_flat[sub_i], num, start, end
            ))?;
            print!("{insert}");
            if data_flat[add_i].is_empty() {
                data_flat[add_i] = String::from(insert);
            } else {
                data_flat[add_i].push(insert);
            }
        }
        println!();
        println!("{:?}", data_flat);
    }

    Ok(data_flat)
}

fn fmt_ins(ins: &str) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    ins.split('\n')
        .filter(|&x| !x.is_empty())
        .map(|line| ins_reg_helper(line).unwrap())
    // .collect::<Vec<(i32,i32, i32)>>()
}

fn ins_reg_helper(str: &str) -> Result<(i32, i32, i32), Box<dyn std::error::Error>> {
    lazy_static! {
        static ref REG: Regex = Regex::new(r"\d+").unwrap();
    }
    // println!("{}", str);
    let mut gen = REG
        .find_iter(str)
        // we are matching on \d+ which will only give me digits, fine to do the unwrap without checking
        .map(|mat| mat.as_str().parse::<i32>().unwrap());
    // Assume that we will always have 3 numbers in each line, other things are broken if we don't
    Ok((
        gen.next().ok_or("Bad pos!")?,
        gen.next().ok_or("Bad start")?,
        gen.next().ok_or("Bad end")?,
    ))
}

fn get_data(data: &str) -> Vec<Vec<&str>> {
    println!("{data:?}");
    data.rsplit('\n')
        .filter(regex_helper)
        .map(|line| line_helper(line).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn fmt_data(data: &str) -> (Vec<Vec<&str>>, Vec<String>) {
    let x = get_data(data);
    println!("{:?}", x);
    let size = x
        .iter()
        .fold(0, |acc, arr| if acc > arr.len() { acc } else { arr.len() });
    let mut v: Vec<String> = vec![String::new(); size];
    for level in x.iter() {
        for (i, cargo_crate) in level.iter().enumerate() {
            if cargo_crate.is_empty() {
                continue;
            }
            v[i].push_str(cargo_crate);
        }
    }
    (x, v)
}

fn regex_helper(str: &&str) -> bool {
    lazy_static! {
        static ref REG: Regex = Regex::new(r"\d").unwrap();
    }
    // let ret =!REG.is_match(str);
    // println!("REGEX_HELPER: STR: {}, RET: {}",str, ret);
    // ret
    !REG.is_match(str)
}
fn line_helper(str: &str) -> impl Iterator<Item = &str> {
    lazy_static! {
        static ref REG: Regex = Regex::new(r"\s+").unwrap();
    }
    println!("LINE_HELPER: STR: {}", str);

    str.split(&['[', ']'])
        .map(|str| {
            println!("BEFORE FILTER {str:?}");
            str
        })
        .filter(|line| !(line.is_empty() || *line == " "))
        .map(|str| {
            println!("LINE_HELPER: ITERATOR: STR: {}", str);
            if REG.is_match(str) {
                ""
            } else {
                str
            }
        })
}

fn get_data_test(data: &str) -> Vec<Vec<String>> {
    println!("{data:?}");
    data.rsplit('\n')
        .filter(regex_helper)
        .map(line_helper_test)
        .collect::<Vec<Vec<String>>>()
}

// impl Iterator<Item = &str>
fn line_helper_test(str: &str) -> Vec<String> {
    lazy_static! {
        static ref REG: Regex = Regex::new(r"\s+").unwrap();
    }
    println!("LINE_HELPER: STR: {:?}", str);
    // every 4 chars, since we capture the space on every char, and also
    let char_list = str.chars().collect::<Vec<char>>();
    let x = char_list
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 4 == 0 && i != 0)
        .map(|(i, _)| {
            let candidate = &char_list[i - 3];
            println!("{candidate}");
            // println!("HELPER: {string:?}");
            // if REG.is_match(string.as_str()) {
            //     ""
            // } else {
            //
            // }
            if candidate.is_alphabetic() {
                String::from(*candidate)
            } else {
                String::from("")
            }
        })
        .collect::<Vec<String>>();
    println!("{x:?}");
    x
    // let x = (&char_list).iter().enumerate().nth(3).map(|(i, m)| {
    //     let string = String::from_iter(&char_list[i-3..i]);
    //     println!("{string}");
    //     string
    // }).collect::<Vec<String>>();
}

fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path)?.parse()?)
}

// fn create_coord<'a>(start: &'a i32, end: &'a i32) -> impl Iterator<Item = (i32, i32)> + 'a {
//     if start > end {
//         let (start, end) = (end, start);
//     }
//     (*start..=*end).map(|test| (test, *end-test))
// }

// Puzzle input is going to be inserted as this
// just like day 1 we can cut on double newline, separates the data from instructions
// we use a vec of strings to insert characters in each slot, we can know how many items in the
// vector by reading the last number in the last line of the data.
// we can parse the numbers and remove 1 from them to then
// we then do a reverse search, filter both [], space and number.
//     [D]
// [N] [C]
// [Z] [M] [P]

//  1   2   3
//
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

#[cfg(test)]
mod tests {
    use crate::reader;
    use lazy_static::lazy_static;

    // previous, current, expected
    #[test]
    fn sample() -> Result<(), Box<dyn std::error::Error>> {
        lazy_static! {
            // looking at cargo expand, since this is a reference, it cannot be converted to a string literal
            // The deref can be formatted as a string literal since this now points to the actual vector
            static ref EXPECTED: Vec<String> = vec![String::from("C"), String::from("M"), String::from("PDNZ")];

        }
        let path = "../sample.txt";
        let output = crate::part_1(path)?;
        // Do a check here if every string is equal to the other, also since `all()` will be true on
        // empty arrays, do an and operation to make sure we get false on empty lines.
        let check = output
            .iter()
            .enumerate()
            .map(|(i, c)| c.eq(&EXPECTED[i]))
            .all(|b| b)
            && !output.is_empty();
        assert!(
            check,
            "EXPECTED {:?} does not equal result {:?}",
            *EXPECTED, output
        );
        Ok(())
    }
    #[test]
    fn input() -> Result<(), Box<dyn std::error::Error>> {
        let expected: [String; 9] = [
            "NBDTVGZJ", "SRMDWPF", "VCRSZ", "RTJZPHG", "TCJNDZQF", "NVPWGSFM", "GCVBPQ", "ZBPN",
            "WPJ",
        ]
        .map(String::from);
        let path = "../input.txt";
        let input = reader(path)?;
        let raw_data = input.split("\n\n").collect::<Vec<&str>>();
        let output = crate::get_data_test(raw_data[0]);
        println!("{output:?}");
        // let (output,flat_output) = crate::fmt_data(raw_data[0]);
        // println!("RAWDATA:\n{:?}\nOUT:\n\n{:?}\nFLATOUT:\n\n{:?}\n{:?}", raw_data[0], output, flat_output, expected);
        // for (i, string) in expected.iter().enumerate() {
        //     assert_eq!(flat_output[i], *string, "EXPECTED {:?} does not equal result {:?}", flat_output[i], *string);
        // }

        Ok(())
    }
}
