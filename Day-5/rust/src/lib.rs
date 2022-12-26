use lazy_static::lazy_static;
use regex::Regex;
use std::mem;

struct CargoData<'a> {
    raw_string: &'a str,
    lines: Option<Vec<Vec<char>>>,
    lines_flat: Option<Vec<Vec<char>>>,
    lines_char_flat: Option<Vec<Vec<char>>>,
    lines_string_flat: Option<Vec<String>>,
}

impl<'a> CargoData<'a> {
    pub fn new(raw_string: &'a str) -> Self {
        Self {
            raw_string,
            lines: None,
            lines_flat: None,
            lines_char_flat: None,
            lines_string_flat: None,
        }
    }
    pub fn build_lines(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_val: usize = 0;
        let chars = self.raw_string.chars().collect::<Vec<char>>();
        self.lines = Some(
            chars
                .iter()
                .enumerate()
                .filter(|&(_, &char)| char == '\n')
                .map(|(i, _)| {
                    let char_array = Vec::from(&chars[last_val..=i]);
                    // reminder that if i is pointing to the newline, which we dont want
                    // capture here.
                    last_val = i + 1;
                    char_array
                })
                .collect(),
        );

        Ok(())
    }
    pub fn build_flat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Dont
        let char_list = self.lines.as_ref().ok_or("")?;
        self.lines_flat = Some(
            char_list
                .iter()
                .enumerate()
                .map(|(i0, list_char_list)| {
                    list_char_list
                        .iter()
                        .enumerate()
                        .filter(|&(i1, _)| {
                            // print!("{i1} ");
                            (i1 + 1) % 4 == 0 && i1 != 0
                        })
                        .map(|(i1, _)| {
                            // println!("\n");
                            // println!("I: {i1}");
                            let x = char_list[i0][i1 - 2];
                            // println!("LISTCHARLIST: {x:?} FULL {:?} {}", &char_list[i0][i1-3..i1], char_list[i0].len());
                            if x.is_alphabetic() {
                                x
                            } else {
                                '\0'
                            }
                        })
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
        );

        Ok(())
    }
    pub fn fmt_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let lines = self.lines_flat.as_ref().ok_or("")?;
        // let mut v: Vec<String> = vec![String::with_capacity(lines.len()); lines[0].len()];
        let mut v: Vec<Vec<char>> = vec![Vec::with_capacity(lines.len()); lines[0].len()];
        for level in lines {
            for (i, &cargo_crate) in level.iter().enumerate() {
                if cargo_crate == '\0' {
                    continue;
                }
                v[i].push(cargo_crate);
            }
        }
        self.lines_char_flat = Some(v);
        // self.lines_char_flat = (0..lines[0].len()).map(|i| (0..lines.len()).map())
        Ok(())
    }
    pub fn build_strings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.lines_string_flat = Some(
            self.lines_char_flat
                .as_ref()
                .ok_or("")?
                .iter()
                .map(|chars| chars.iter().rev().collect::<String>())
                .collect(),
        );
        Ok(())
    }
    pub fn move_cargo(&mut self, start: usize, end: usize) {
        let lines_mut = self.lines_string_flat.as_mut().unwrap();
        let tmp = lines_mut[start].pop().unwrap();
        lines_mut[end].push(tmp);
    }
    pub fn solution(&self) {
        let end = self
            .lines_string_flat
            .as_ref()
            .unwrap()
            .iter()
            .flat_map(|row| {
                row.chars()
                    .enumerate()
                    .filter(|&(i, _)| i == row.len() - 1)
                    .map(|(_, c)| c)
            })
            .collect::<String>();
        println!("The solution is {}", end);
    }
    pub fn verify(&self, source: &[String]) -> bool {
        let lines = self.lines_string_flat.as_ref().unwrap();
        if lines.len() != source.len() {
            return false;
        }
        lines.iter().enumerate().all(|(i, line)| *line == source[i])
    }

    fn head_tail_iterator<F>(string: &'a str, func: F) -> impl Iterator<Item = char> + '_
    where
        F: FnMut(&(usize, char)) -> bool + 'a,
    {
        string.chars().enumerate().filter(func).map(|(_, c)| c)
    }

    pub fn move_cargo_9001(&mut self, start: usize, end: usize, amount: usize) {
        let lines_mut = self.lines_string_flat.as_mut().unwrap();
        let (head, tail) = if lines_mut[start].len() - amount == 0 {
            // TODO: take here swaps an empty string in the start, and then gives me the old string back
            ("".to_string(), mem::take(&mut lines_mut[start]))
        } else {
            (
                // 0..2-1
                // 1
                // lines_mut[start].chars()
                //     .enumerate()
                //     .filter(|(i, _)  | *i < lines_mut[start].len() - amount)
                //     .map(|(_, c)| c)
                //     .collect::<String>(),
                CargoData::head_tail_iterator(&lines_mut[start], |&(i, _)| {
                    i < lines_mut[start].len() - amount
                })
                .collect::<String>(),
                CargoData::head_tail_iterator(&lines_mut[start], |&(i, _)| {
                    i >= lines_mut[start].len() - amount
                })
                .collect::<String>(), // lines_mut[start][0..lines_mut.len() - amount].to_owned(),
                                      // lines_mut[start][lines_mut.len() - amount..lines_mut.len()].to_owned(),
            )
        };
        // println!("HEAD: {:?} TAIL: {:?} ", head, tail);
        lines_mut[end].push_str(tail.as_str());

        lines_mut[start] = head;
        // println!("STATE_NEW: {:?} ", lines_mut);
    }
    pub fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.build_lines()?;
        self.build_flat()?;
        self.fmt_data()?;
        self.build_strings()?;
        Ok(())
    }

    // This does not mimic real life situation, the Cargo does not move itself, the Crane operator does
    // pub fn exec_ins(&mut self, amount: i32, start: i32, end: i32) -> Result<(), Box<dyn std::error::Error>> {
    //     let lines = self.lines_string_flat.as_mut().ok_or("Failed to read Vector lines_string_flat")?;
    //     (0..=amount).for_each(|_| {
    //         let tmp = lines[start as usize].pop().unwrap_or_default();
    //         lines[end as usize].push(tmp);
    //     });
    //     Ok(())
    // }
}

struct CargoInstructions<'a> {
    raw_string: &'a str,
    instruction: Vec<(usize, usize, usize)>,
    // amount: Vec<i32>,
    // start: Vec<i32>,
    // end: Vec<i32>,
}

impl<'a> CargoInstructions<'a> {
    fn new(raw_string: &'a str) -> Self {
        Self {
            raw_string,
            instruction: Vec::new(),
        }
    }
    fn build_helper(str: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        let map = str
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(CargoInstructions::line_helper);
        map
    }
    fn line_helper(line: &str) -> (usize, usize, usize) {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"\d+").unwrap();
        }
        let mut generator = REG
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<usize>().unwrap());
        (
            generator.next().unwrap(),
            generator.next().unwrap() - 1,
            generator.next().unwrap() - 1,
        )
    }
    pub fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for ins in CargoInstructions::build_helper(self.raw_string) {
            self.instruction.push(ins);
        }
        Ok(())
    }
    pub fn exec(&self, cargo: &mut CargoData) -> Result<(), Box<dyn std::error::Error>> {
        self.instruction
            .iter()
            .for_each(|(pos, start, end)| (0..*pos).for_each(|_| cargo.move_cargo(*start, *end)));
        Ok(())
    }
    pub fn exec_pt2(&self, cargo: &mut CargoData) -> Result<(), Box<dyn std::error::Error>> {
        // self.instruction.iter()
        //     .for_each(|&(pos, start, end)|
        //         cargo.move_cargo_9001(start, end, pos)
        for &(pos, start, end) in self.instruction.iter() {
            cargo.move_cargo_9001(start, end, pos);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{CargoData, CargoInstructions};
    use std::fs;

    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }
    fn initial(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let input = reader(path)?;
        Ok(input
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>())
    }

    #[test]
    fn sample() -> Result<(), Box<dyn std::error::Error>> {
        let raw_data = initial("../sample.txt")?;
        let solution: [String; 3] = ["C", "M", "PDNZ"].map(String::from);
        let expected: [String; 3] = ["ZN", "MCD", "P"].map(String::from);
        let mut cargodata = CargoData::new(raw_data[0].as_str());

        cargodata.build_lines()?;
        cargodata.build_flat()?;
        cargodata.fmt_data()?;
        cargodata.build_strings()?;
        assert!(
            cargodata.verify(&expected),
            "Expected Cargo Data {:?} does not match actual {:?}",
            expected,
            cargodata.lines_string_flat
        );
        let mut instructions = CargoInstructions::new(raw_data[1].as_str());
        instructions.build()?;
        instructions.exec(&mut cargodata)?;
        assert!(
            cargodata.verify(&solution),
            "Expected Solution state for Cargo sample {:?} does not match actual {:?}",
            solution,
            cargodata.lines_string_flat
        );
        // println!("{:?}", cargodata.lines_char_flat);
        cargodata.solution();
        // CargoData::fmt_data(cargo);
        Ok(())
    }
    #[test]
    fn input() -> Result<(), Box<dyn std::error::Error>> {
        // Solution is GFTNRBZPF
        let expected: [String; 9] = [
            "NBDTVGZJ", "SRMDWPF", "VCRSZ", "RTJZPHG", "TCJNDZQF", "NVPWGSFM", "GCVBPQ", "ZBPN",
            "WPJ",
        ]
        .map(String::from);
        let raw_data = initial("../input.txt")?;
        let mut cargodata = CargoData::new(raw_data[0].as_str());
        cargodata.build_lines()?;
        cargodata.build_flat()?;
        cargodata.fmt_data()?;
        cargodata.build_strings()?;
        assert!(
            cargodata.verify(&expected),
            "Expected Cargo Data {:?} does not match actual {:?}",
            expected,
            &cargodata.lines_string_flat
        );
        let mut instructions = CargoInstructions::new(raw_data[1].as_str());
        instructions.build()?;
        instructions.exec(&mut cargodata)?;
        cargodata.solution();
        Ok(())
    }
    #[test]
    fn input_part2() -> Result<(), Box<dyn std::error::Error>> {
        let raw_data = initial("../input.txt")?;
        let mut cargodata = CargoData::new(raw_data[0].as_str());
        let mut instructions = CargoInstructions::new(raw_data[1].as_str());
        cargodata.build()?;
        instructions.build()?;
        instructions.exec_pt2(&mut cargodata)?;
        cargodata.solution();
        Ok(())
    }
    #[test]
    fn sample_part2() -> Result<(), Box<dyn std::error::Error>> {
        // solution is VRQWPDSGP
        let raw_data = initial("../sample.txt")?;
        let mut cargodata = CargoData::new(raw_data[0].as_str());
        let mut instructions = CargoInstructions::new(raw_data[1].as_str());
        cargodata.build()?;
        instructions.build()?;
        instructions.exec_pt2(&mut cargodata)?;
        cargodata.solution();
        Ok(())
    }
}
