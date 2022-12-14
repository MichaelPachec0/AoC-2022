mod structs;
use structs::error::ComputeErr;

fn compute(input: &str, size: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let chars = input.chars().collect::<Vec<char>>();
    // The compiler is not smart enough to infer here, so we need to define types for closures
    let guard = |&(i, _): &(usize, &char)| i + size <= chars.len();
    for (i0, _) in chars.iter().enumerate().filter(guard) {
        let slice: &[char] = &chars[i0..i0 + size];
        // println!("CHECK FOR CHAR {} AT {} WITH CHAR {} AT {} {}", char0, i1, char1, i2, char0 != char1);
        if slice.iter().enumerate().all(|(i1, &char0)| {
            slice
                .iter()
                .enumerate()
                .all(|(i2, &char1)| i1 == i2 || char0 != char1)
        }) {
            return Ok(i0 + size);
        }
        continue;
    }
    Err(Box::try_from(ComputeErr::new(
        format!("No valid string slice for size {size} found!").as_str(),
    ))?)
}

/// # Errors
///
/// Will return an `Err` if there is not a valid slice with the constrains given in the `size` parameter
/// given. In this case it is 4 chars.
pub fn exec_pt1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    compute(input, 4)
}
/// # Errors
///
/// Will return an `Err` if there is not a valid slice with the constrains given in the `size` parameter
/// given. In this case it is 14 chars.
pub fn exec_pt2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    compute(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }
    fn get_input(raw: &str) -> impl Iterator<Item = (&str, usize)> {
        raw.split('\n')
            .filter(|&line| !line.is_empty())
            .map(|line| {
                let mut tmp = line.split(' ');
                (
                    tmp.next().unwrap(),
                    tmp.next().unwrap_or("0").parse::<usize>().unwrap(),
                )
            })
    }

    fn sample_check<F>(lines: &[(&str, usize)], func: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&str) -> Result<usize, Box<dyn std::error::Error>>,
    {
        for (i, &(input, expected)) in lines.iter().enumerate() {
            println!("TEST {i} of {} : STRING: {input}", lines.len() - 1);
            let result = func(input)?;
            assert_eq!(
                expected, result,
                "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}",
                result, input, expected
            );
        }
        Ok(())
    }

    #[test]
    fn sample_simple_pt1() -> Result<(), Box<dyn std::error::Error>> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        // TODO: decide if i want to default to i32 or usize
        let expected = 7;
        let result = exec_pt1(input)?;
        assert_eq!(
            expected, result,
            "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}",
            result, input, expected
        );
        Ok(())
    }
    #[test]
    fn sample_pt1() -> Result<(), Box<dyn std::error::Error>> {
        let raw_input = reader("../sample.txt")?;
        let lines = get_input(raw_input.as_ref()).collect::<Vec<(&str, usize)>>();
        sample_check(&lines, exec_pt1)?;
        Ok(())
    }
    #[test]
    fn sample_end_line_check() -> Result<(), Box<dyn std::error::Error>> {
        let input = "mjqjjjjjjjjjjjjjjjjjjjjpqm";
        let expected = input.len();
        let result = exec_pt1(input)?;
        assert_eq!(
            expected, result,
            "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}",
            result, input, expected
        );
        Ok(())
    }
    #[test]
    fn sample_test_error() {
        let input = "mjqjjjjjjjjjjjjjjjjjjjjjj";
        let expected_err =
            "Err(ComputeError { details: \"No valid string slice for size 4 found!\" })";
        let result = exec_pt1(input);
        let error_str = format!("{result:?}");
        assert!(
            result.is_err() && expected_err == error_str,
            "INPUT {} should have errored. Instead got {:?}.",
            input,
            error_str
        );
    }
    #[test]
    fn input_pt1() -> Result<(), Box<dyn std::error::Error>> {
        let raw_input = reader("../input.txt")?;
        let result = exec_pt1(raw_input.as_str())?;
        println!("The result is {result}");
        Ok(())
    }
    #[test]
    fn sample_pt2() -> Result<(), Box<dyn std::error::Error>> {
        let raw_input = reader("../sample_pt2.txt")?;
        let lines = get_input(raw_input.as_ref()).collect::<Vec<(&str, usize)>>();
        sample_check(&lines, exec_pt2)?;
        Ok(())
    }
    #[test]
    fn input_pt2() -> Result<(), Box<dyn std::error::Error>> {
        let raw_input = reader("../input.txt")?;
        let result = exec_pt2(raw_input.as_ref())?;
        println!("The result is {result}");
        Ok(())
    }
}
