pub fn exec_pt1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for (i0, char) in chars.iter().enumerate() {
        let slice: &[char] = &chars[i0..i0+4];
        if slice.iter().enumerate().all(|(i1,&char0)| slice.iter().enumerate().all(|(i2, &char1)| {
            println!("CHECK FOR CHAR {} AT {} WITH CHAR {} AT {} {}", char0, i1, char1, i2, char0 != char1);
            if i1 == i2 {
                true
            } else {
                char0 != char1
            }
        }
        )) {
            return i0 + 4;
        }
    }
    255_usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_pt1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        // TODO: decide if i want to default to i32 or usize
        let expected = 7;
        let result = exec_pt1(input);
        assert_eq!(expected, result, "RESULT {} FOR INPUT {} DOES NOT EQUAL EXPECTED {}", result, input, expected);
    }
}
