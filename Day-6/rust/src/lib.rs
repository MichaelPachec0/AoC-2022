pub fn exec_pt1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for (i0, _) in chars.iter().enumerate() {
        let slice: &[char] = &chars[i0..i0 + 4];
        if slice.iter().enumerate().all(|(i1, &char0)| {
            slice.iter().enumerate().all(|(i2, &char1)| {
                // println!("CHECK FOR CHAR {} AT {} WITH CHAR {} AT {} {}", char0, i1, char1, i2, char0 != char1);
                if i1 == i2 {
                    true
                } else {
                    char0 != char1
                }
            })
        }) {
            return i0 + 4;
        }
    }
    // Something wrong happened here, there was no unique 4 characters found.
    // TODO: should i make this a proper error? or just send 0 or 255
    255_usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }

    #[test]
    fn sample_simple_pt1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        // TODO: decide if i want to default to i32 or usize
        let expected = 7;
        let result = exec_pt1(input);
        assert_eq!(
            expected, result,
            "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}",
            result, input, expected
        );
    }
    #[test]
    fn sample_pt1() -> Result<(), Box<dyn std::error::Error>> {
        let raw_input = reader("../sample.txt")?;
        let line_input = raw_input
            .split('\n')
            .filter(|&line| !line.is_empty())
            .collect::<Vec<&str>>();
        let input = line_input
            .iter()
            .map(|line| {
                let mut tmp = line.split(' ');
                (
                    tmp.next().unwrap(),
                    tmp.next().unwrap_or("0").parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(&str, usize)>>();
        for (i, &(input, expected)) in input.iter().enumerate() {
            println!("TEST {} of {} : STRING: {}", i, line_input.len() - 1, input);
            let result = exec_pt1(input);
            assert_eq!(
                expected, result,
                "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}",
                result, input, expected
            );
        }
        Ok(())
    }
}
